//! `DevicePath` protocol

use crate::{proto::Protocol, unsafe_guid};

/// DevicePath protocol. This can be opened on a `LoadedImage.device()` handle
/// using the `HandleProtocol` boot service.
#[repr(C)]
#[unsafe_guid("09576e91-6d3f-11d2-8e39-00a0c969723b")]
#[derive(Protocol)]
pub struct DevicePath {
    /// Type of device
    pub device_type: DeviceType,
    /// Sub type of device
    pub sub_type: DeviceSubType,
    /// Data related to device path
    ///
    /// The device_type and sub_type determine the
    /// kind of data, and its size.
    pub length: [u8; 2],
}

/// Type identifier for a DevicePath
#[repr(u8)]
#[derive(Debug)]
pub enum DeviceType {
    Hardware = 0x01,
    Acpi = 0x02,
    Messaging = 0x03,
    Media = 0x04,
    BiosBootSpec = 0x05,
    End = 0x7F,
}

/// Sub-type identifier for a DevicePath
#[repr(u8)]
#[derive(Debug)]
pub enum DeviceSubType {
    EndInstance = 0x01,
    EndEntire = 0xFF,
}
