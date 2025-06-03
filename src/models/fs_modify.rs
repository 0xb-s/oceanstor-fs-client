use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct ModifyFsReq<'a> {
    #[serde(rename = "NAME", skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    #[serde(rename = "DESCRIPTION", skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    #[serde(rename = "CAPACITY", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<u64>,

    #[serde(rename = "SNAPSHOTRESERVEPER", skip_serializing_if = "Option::is_none")]
    pub snapshot_reserve_per: Option<u32>,

    #[serde(
        rename = "AUTODELSNAPSHOTENABLE",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_del_snapshot: Option<bool>,

    #[serde(rename = "CAPACITYTHRESHOLD", skip_serializing_if = "Option::is_none")]
    pub capacity_threshold: Option<u64>,

    #[serde(rename = "SECTORSIZE", skip_serializing_if = "Option::is_none")]
    pub sector_size: Option<u32>,

    #[serde(rename = "OWNINGCONTROLLER", skip_serializing_if = "Option::is_none")]
    pub owning_controller: Option<&'a str>,

    #[serde(rename = "CHECKSUMENABLE", skip_serializing_if = "Option::is_none")]
    pub checksum_enable: Option<bool>,

    #[serde(rename = "ATIME", skip_serializing_if = "Option::is_none")]
    pub atime: Option<bool>,

    #[serde(rename = "READONLY", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(rename = "IOPRIORITY", skip_serializing_if = "Option::is_none")]
    pub io_priority: Option<u8>, // 1=low, 2=mid, 3=high

    #[serde(rename = "ENABLECOMPRESSION", skip_serializing_if = "Option::is_none")]
    pub enable_compression: Option<bool>,

    #[serde(rename = "COMPRESSION", skip_serializing_if = "Option::is_none")]
    pub compression: Option<u8>, // 0=rapid, 1=deep

    #[serde(rename = "ENABLEDEDUP", skip_serializing_if = "Option::is_none")]
    pub enable_dedup: Option<bool>,

    #[serde(
        rename = "WORMMINPROTECTPERIOD",
        skip_serializing_if = "Option::is_none"
    )]
    pub worm_min_protect_period: Option<u64>,

    #[serde(
        rename = "WORMMAXPROTECTPERIOD",
        skip_serializing_if = "Option::is_none"
    )]
    pub worm_max_protect_period: Option<u64>,

    #[serde(
        rename = "WORMDEFPROTECTPERIOD",
        skip_serializing_if = "Option::is_none"
    )]
    pub worm_def_protect_period: Option<u64>,

    #[serde(rename = "WORMAUTOLOCK", skip_serializing_if = "Option::is_none")]
    pub worm_auto_lock: Option<bool>,

    #[serde(rename = "WORMAUTOLOCKTIME", skip_serializing_if = "Option::is_none")]
    pub worm_auto_lock_time: Option<u32>, // 2h to 7d

    #[serde(rename = "WORMAUTODEL", skip_serializing_if = "Option::is_none")]
    pub worm_auto_del: Option<bool>,

    #[serde(rename = "WRITECHECK", skip_serializing_if = "Option::is_none")]
    pub write_check: Option<bool>,

    #[serde(
        rename = "TIMINGSNAPSHOTMAXNUM",
        skip_serializing_if = "Option::is_none"
    )]
    pub timing_snapshot_max_num: Option<u32>,

    #[serde(rename = "ISSHOWSNAPDIR", skip_serializing_if = "Option::is_none")]
    pub is_show_snap_dir: Option<bool>,

    #[serde(rename = "DATATRANSFERPOLICY", skip_serializing_if = "Option::is_none")]
    pub data_transfer_policy: Option<u8>, // 0-3

    #[serde(
        rename = "ENABLETIMINGSNAPSHOT",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_timing_snapshot: Option<bool>,

    #[serde(rename = "SMARTCACHESTATE", skip_serializing_if = "Option::is_none")]
    pub smart_cache_state: Option<u8>, // 0-2

    #[serde(rename = "MINPROTECTTIMEUNIT", skip_serializing_if = "Option::is_none")]
    pub min_protect_time_unit: Option<u8>, // 46-48

    #[serde(rename = "MAXPROTECTTIMEUNIT", skip_serializing_if = "Option::is_none")]
    pub max_protect_time_unit: Option<u8>,

    #[serde(rename = "DEFPROTECTTIMEUNIT", skip_serializing_if = "Option::is_none")]
    pub def_protect_time_unit: Option<u8>,

    #[serde(rename = "AUTOLOCKTIMEUNIT", skip_serializing_if = "Option::is_none")]
    pub auto_lock_time_unit: Option<u8>,

    #[serde(rename = "vstoreId", skip_serializing_if = "Option::is_none")]
    pub vstore_id: Option<&'a str>,

    #[serde(
        rename = "SPACESELFADJUSTINGMODE",
        skip_serializing_if = "Option::is_none"
    )]
    pub space_self_adjust_mode: Option<u8>,

    #[serde(rename = "AUTOSIZEENABLE", skip_serializing_if = "Option::is_none")]
    pub auto_size_enable: Option<u8>,

    #[serde(
        rename = "AUTOSHRINKTHRESHOLDPERCENT",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_shrink_threshold_percent: Option<u32>,

    #[serde(
        rename = "AUTOGROWTHRESHOLDPERCENT",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_grow_threshold_percent: Option<u32>,

    #[serde(rename = "MINAUTOSIZE", skip_serializing_if = "Option::is_none")]
    pub min_auto_size: Option<u64>,

    #[serde(rename = "MAXAUTOSIZE", skip_serializing_if = "Option::is_none")]
    pub max_auto_size: Option<u64>,

    #[serde(rename = "AUTOSIZEINCREMENT", skip_serializing_if = "Option::is_none")]
    pub auto_size_increment: Option<u64>,

    #[serde(rename = "SPACERECYCLEMODE", skip_serializing_if = "Option::is_none")]
    pub space_recycle_mode: Option<u8>,

    #[serde(rename = "ALLOCTYPE", skip_serializing_if = "Option::is_none")]
    pub alloc_type: Option<u8>,

    #[serde(rename = "SPLITSPEED", skip_serializing_if = "Option::is_none")]
    pub split_speed: Option<u8>, // 1-4

    #[serde(
        rename = "ALTERNATEDATASTREAMS",
        skip_serializing_if = "Option::is_none"
    )]
    pub alternate_data_streams: Option<u8>, // 0=no, 1=yes

    #[serde(
        rename = "SSDCAPACITYUPPERLIMIT",
        skip_serializing_if = "Option::is_none"
    )]
    pub ssd_capacity_upper_limit: Option<u64>,

    #[serde(rename = "securityStyle", skip_serializing_if = "Option::is_none")]
    pub security_style: Option<u8>, // 0=mixed, 1=native, 2=ntfs, 3=unix

    #[serde(
        rename = "enableBgrCompression",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_bgr_compression: Option<u8>, // 0=off, 1=on

    #[serde(rename = "enableBgrDedup", skip_serializing_if = "Option::is_none")]
    pub enable_bgr_dedup: Option<u8>, // 0=off, 1=on
}
