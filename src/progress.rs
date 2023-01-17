//! SWUpdate progress API

use enum_primitive::*;
use libswupdate_sys::*;
use std::ffi::{CStr, CString};
use std::io;
use std::mem::MaybeUninit;
use std::os::fd::RawFd;

/// Message sent via progress socket.
#[derive(Debug, Clone)]
pub struct ProgressMsg {
    /// Update Status (Running, Failure)
    pub status: RecoveryStatus,
    /// % downloaded data
    pub dwl_percent: u8,
    /// Total of bytes to be downloaded
    pub dwl_bytes: u64,
    /// No. total of steps
    pub nsteps: u32,
    /// Current step index
    pub cur_step: u32,
    /// % in current step
    pub cur_percent: u8,
    /// Name of image to be installed
    pub cur_image: Option<String>,
    /// Name of running handler
    pub hnd_name: Option<String>,
    /// Interface that triggered the update
    pub source: SourceType,
    /// Additional information about install
    pub info: Option<String>,
}

impl From<progress_msg> for ProgressMsg {
    fn from(m: progress_msg) -> Self {
        Self {
            status: RecoveryStatus::from_u32(m.status).unwrap_or_default(),
            dwl_percent: m.dwl_percent as u8,
            dwl_bytes: m.dwl_bytes,
            nsteps: m.nsteps,
            cur_step: m.cur_step,
            cur_percent: m.cur_percent as u8,
            cur_image: if m.cur_image[0] == 0 {
                None
            } else {
                Some(
                    unsafe { CStr::from_ptr(m.cur_image.as_ptr()) }
                        .to_string_lossy()
                        .to_string(),
                )
            },
            hnd_name: if m.hnd_name[0] == 0 {
                None
            } else {
                Some(
                    unsafe { CStr::from_ptr(m.hnd_name.as_ptr()) }
                        .to_string_lossy()
                        .to_string(),
                )
            },
            source: SourceType::from_u32(m.source).unwrap_or_default(),
            info: if m.info[0] == 0 {
                None
            } else {
                Some(
                    unsafe { CStr::from_ptr(m.info.as_ptr()) }
                        .to_string_lossy()
                        .to_string(),
                )
            },
        }
    }
}

#[derive(Default)]
pub struct SWUpdateProgress {
    fd: Option<RawFd>,
}

impl Drop for SWUpdateProgress {
    fn drop(&mut self) {
        if let Some(fd) = self.fd {
            unsafe {
                close(fd);
            }
        }
    }
}

impl SWUpdateProgress {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the SWUpdate progress socket path.
    pub fn get_socket_path(&self) -> String {
        let test = unsafe { CStr::from_ptr(get_prog_socket()) };
        String::from(test.to_str().unwrap())
    }

    /// Connect to the default progress interface.
    pub fn connect(&mut self) -> Result<(), io::Error> {
        let fd = unsafe { progress_ipc_connect(false) };
        if fd == -1 {
            return Err(io::Error::last_os_error());
        }
        self.fd = Some(fd);
        Ok(())
    }

    /// Connect to the given progress interface.
    ///
    /// In case more as an instance of SWUpdate is running, this allows to select which should be taken.
    pub fn connect_with_path(&mut self, socket_path: impl AsRef<str>) -> Result<(), io::Error> {
        let socket_path = CString::new(socket_path.as_ref())
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?;
        let fd = unsafe { progress_ipc_connect_with_path(socket_path.as_ptr(), false) };
        if fd == -1 {
            return Err(io::Error::last_os_error());
        }
        self.fd = Some(fd);
        Ok(())
    }

    /// Retrieve messages from progress interface.
    ///
    /// This is a blocking operation.
    pub fn receive(&mut self) -> Result<ProgressMsg, io::Error> {
        if let Some(fd) = self.fd.as_mut() {
            let mut msg: MaybeUninit<progress_msg> = MaybeUninit::uninit();

            let ret = unsafe { progress_ipc_receive(fd, msg.as_mut_ptr()) };

            if ret <= 0 {
                self.fd = None; // already closed by library
                Err(io::Error::last_os_error())
            } else {
                let mut msg = unsafe { msg.assume_init() };
                // swupdate-progress.c has this comment: "Be sure that string in message are Null terminated". So better be safe....
                // Note: len() returns the static array length, not a string length.
                msg.cur_image[msg.cur_image.len() - 1] = 0;
                msg.hnd_name[msg.hnd_name.len() - 1] = 0;
                msg.info[msg.info.len() - 1] = 0;
                Ok(msg.into())
            }
        } else {
            Err(io::Error::from(io::ErrorKind::NotConnected))
        }
    }
}

use crate::{RecoveryStatus, SourceType};
use std::os::raw::c_int;

// we could also import libc, but for just one function this seems to be overkill
extern "C" {
    pub fn close(fd: c_int) -> c_int;
}
