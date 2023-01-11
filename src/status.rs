//! Rust enumerations adapted from [swupdate_status.h](https://github.com/sbabic/swupdate/blob/master/include/swupdate_status.h)

use enum_primitive::*;
use libswupdate_sys::*;

enum_from_primitive! {
/// This is used to send back the result of an update.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum RecoveryStatus {
    Idle = RECOVERY_STATUS_IDLE,
    Start = RECOVERY_STATUS_START,
    Run = RECOVERY_STATUS_RUN,
    Success = RECOVERY_STATUS_SUCCESS,
    Failure = RECOVERY_STATUS_FAILURE,
    Download = RECOVERY_STATUS_DOWNLOAD,
    Done = RECOVERY_STATUS_DONE,
    SubProcess = RECOVERY_STATUS_SUBPROCESS,
    Progress = RECOVERY_STATUS_PROGRESS,
    #[default]
    Unknown = 255,
}
}

enum_from_primitive! {
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum SourceType {
    #[default]
    Unknown = sourcetype_SOURCE_UNKNOWN,
    WebServer = sourcetype_SOURCE_WEBSERVER,
    Suricatta = sourcetype_SOURCE_SURICATTA,
    Downloader = sourcetype_SOURCE_DOWNLOADER,
    Local = sourcetype_SOURCE_LOCAL,
    ChunksDownloader = sourcetype_SOURCE_CHUNKS_DOWNLOADER,
}
}
