//! SWUpdate IPC API

use crate::SourceType;
use enum_primitive::*;
use libswupdate_sys::*;

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum MsgType {
        ReqInstall = msgtype_REQ_INSTALL,
        Ack = msgtype_ACK,
        Nack = msgtype_NACK,
        GetStatus = msgtype_GET_STATUS,
        PostUpdate = msgtype_POST_UPDATE,
        SWUpdateSubprocess = msgtype_SWUPDATE_SUBPROCESS,
        SetAesKey = msgtype_SET_AES_KEY,
        /// set bootloader ustate
        SetUpdateState = msgtype_SET_UPDATE_STATE,
        GetUpdateState = msgtype_GET_UPDATE_STATE,
        ReqInstallExt = msgtype_REQ_INSTALL_EXT,
        SetVersionsRange = msgtype_SET_VERSIONS_RANGE,
        NotifyStream = msgtype_NOTIFY_STREAM,
        GetHwRevision
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum Command {
        /// this returns the answer if a SW can be activated
        Activation = CMD_ACTIVATION,
        Config = CMD_CONFIG,
        /// Enable or disable suricatta mode
        Enable = CMD_ENABLE,
        GetStatus = CMD_GET_STATUS,
    }
}

enum_from_primitive! {
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum RunType {
        #[default]
        Default = run_type_RUN_DEFAULT,
        DryRun = run_type_RUN_DRYRUN,
        Install = run_type_RUN_INSTALL
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub apiversion: u32,
    pub source: SourceType,
    pub dry_run: RunType,
    pub len: usize,
    pub info: Option<String>,
    pub software_set: Option<String>,
    pub running_mode: Option<String>,
    pub disable_store_swu: bool,
}
