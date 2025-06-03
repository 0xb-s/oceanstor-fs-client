use crate::{
    error::OceanStorError,
    models::{
        common::Count,
        filesystem::{FileSystem, FileSystemSummary},
        fs_clone_split::CloneSplitReq,
        fs_create::{CreateCloneReq, CreateFsReq},
        fs_modify::ModifyFsReq,
        fs_share_cfg::FileShareCfg,
        fs_share_type_cfg::{FileShareTypeCfg, ShareProtocol},
        fs_snapshot::SnapshotToggleReq,
        fs_type_cfg::{FsCfgAbility, FsType, FsTypeCfg},
    },
    util,
};
use reqwest::{header, Client as HttpClient, Method};
use serde::Serialize;

#[derive(Debug)]
pub struct OceanStor {
    http: HttpClient,
    base_url: String,
    device_id: String,
    session_key: Option<String>,
}

impl OceanStor {
    pub fn builder(ip: &str, port: u16) -> OceanStorBuilder {
        OceanStorBuilder::new(ip, port)
    }

    async fn request<R, B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<R, OceanStorError>
    where
        R: serde::de::DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!(
            "{}/deviceManager/rest/{}/{}",
            self.base_url, self.device_id, path
        );
        let mut req = self.http.request(method, &url);

        if let Some(cookie) = &self.session_key {
            req = req.header(header::COOKIE, format!("session={cookie}"));
        }
        if let Some(b) = body {
            req = req.json(b);
        }

        let resp = req.send().await?;
        util::parse(resp).await
    }

    pub async fn login(&mut self, user: &str, password: &str) -> Result<(), OceanStorError> {
        #[derive(Serialize)]
        struct Creds<'a> {
            #[serde(rename = "username")]
            user: &'a str,
            #[serde(rename = "password")]
            pass: &'a str,
        }

        let resp = self
            .http
            .post(format!("{}/deviceManager/rest/sessions", self.base_url))
            .json(&Creds {
                user,
                pass: password,
            })
            .send()
            .await?;

        if let Some(cookie) = resp
            .headers()
            .get(header::SET_COOKIE)
            .and_then(|v| v.to_str().ok())
            .and_then(|h| h.split(';').next())
            .and_then(|kv| kv.strip_prefix("session="))
        {
            self.session_key = Some(cookie.to_owned());
            Ok(())
        } else {
            Err(OceanStorError::Api {
                code: -1,
                description: "login failed – no session cookie".into(),
            })
        }
    }

    pub async fn create_file_system<'a>(
        &self,
        req: CreateFsReq<'a>,
    ) -> Result<FileSystem, OceanStorError> {
        self.request::<FileSystem, _>(Method::POST, "filesystem", Some(&req))
            .await
    }

    pub async fn create_clone_file_system<'a>(
        &self,
        req: CreateCloneReq<'a>,
    ) -> Result<FileSystem, OceanStorError> {
        self.request::<FileSystem, _>(Method::POST, "filesystem", Some(&req))
            .await
    }

    pub async fn list_file_systems(
        &self,
        query: Option<&str>,
    ) -> Result<Vec<FileSystemSummary>, OceanStorError> {
        let path = query
            .map(|q| format!("filesystem?{q}"))
            .unwrap_or_else(|| "filesystem".into());
        self.request::<Vec<FileSystemSummary>, ()>(Method::GET, &path, None)
            .await
    }

    pub async fn get_file_system(&self, fs_id: &str) -> Result<FileSystem, OceanStorError> {
        let path = format!("filesystem/{fs_id}");
        self.request::<FileSystem, ()>(Method::GET, &path, None)
            .await
    }

    pub async fn delete_file_system(&self, fs_id: &str) -> Result<(), OceanStorError> {
        let path = format!("filesystem/{fs_id}");
        self.request::<serde_json::Value, ()>(Method::DELETE, &path, None)
            .await
            .map(|_| ())
    }

    pub async fn modify_file_system<'a>(
        &self,
        fs_id: &str,
        req: ModifyFsReq<'a>,
    ) -> Result<(), OceanStorError> {
        let path = format!("filesystem/{fs_id}");
        self.request::<serde_json::Value, _>(Method::PUT, &path, Some(&req))
            .await
            .map(|_| ())
    }

    pub async fn toggle_clone_split(&self, req: CloneSplitReq<'_>) -> Result<(), OceanStorError> {
        self.request::<serde_json::Value, _>(Method::PUT, "filesystem_split_switch", Some(&req))
            .await
            .map(|_| ())
    }

    pub async fn count_file_systems(&self) -> Result<Count, OceanStorError> {
        self.request::<Count, ()>(Method::GET, "filesystem/count", None)
            .await
    }

    pub async fn fs_type_config(&self, ty: FsType) -> Result<FsTypeCfg, OceanStorError> {
        let path = format!("FILESYSTEM_TYPE_CONFIG/{}", ty as u8);
        self.request::<FsTypeCfg, ()>(Method::GET, &path, None)
            .await
    }

    pub async fn fs_config_ability(&self, ty: FsType) -> Result<FsCfgAbility, OceanStorError> {
        let path = format!("FILESYSTEM_CONFIG_ABILITY/{}", ty as u8);
        self.request::<FsCfgAbility, ()>(Method::GET, &path, None)
            .await
    }

    pub async fn file_share_cfg(&self) -> Result<FileShareCfg, OceanStorError> {
        self.request::<FileShareCfg, ()>(Method::GET, "FILE_SHARE_CONFIG/", None)
            .await
    }

    pub async fn file_share_type_cfg(
        &self,
        proto: ShareProtocol,
    ) -> Result<FileShareTypeCfg, OceanStorError> {
        let path = format!("FILE_SHARE_TYPE_CONFIG/{}", proto as u8);
        self.request::<FileShareTypeCfg, ()>(Method::GET, &path, None)
            .await
    }

    pub async fn set_periodic_snapshot(
        &self,
        req: SnapshotToggleReq<'_>,
    ) -> Result<(), OceanStorError> {
        self.request::<serde_json::Value, _>(
            Method::PUT,
            "filesystem/modify_time_snapshot_status",
            Some(&req),
        )
        .await
        .map(|_| ())
    }
}

#[derive(Debug)]
pub struct OceanStorBuilder {
    ip: String,
    port: u16,
    device_id: Option<String>,
    accept_invalid_certs: bool,
}

impl OceanStorBuilder {
    fn new(ip: &str, port: u16) -> Self {
        Self {
            ip: ip.into(),
            port,
            device_id: None,
            accept_invalid_certs: false,
        }
    }

    pub fn device_id(mut self, id: impl Into<String>) -> Self {
        self.device_id = Some(id.into());
        self
    }

    pub fn disable_certificate_validation(mut self) -> Self {
        self.accept_invalid_certs = true;
        self
    }

    pub fn build(self) -> Result<OceanStor, OceanStorError> {
        let mut http = HttpClient::builder()
            .gzip(true)
            .user_agent("oceanstor-rs/0.1");

        if self.accept_invalid_certs {
            http = http.danger_accept_invalid_certs(true);
        }

        let client = http.build()?;

        Ok(OceanStor {
            http: client,
            base_url: format!("https://{}:{}", self.ip, self.port),
            device_id: self
                .device_id
                .expect("device_id missing - call .device_id()"),
            session_key: None,
        })
    }
}
