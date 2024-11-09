//! Version-information resource. ([`VERSIONINFO`](crate::version_info::VersionInfo))

mod block;
mod fixed;

pub use block::*;
pub use fixed::*;

use crate::{ResError, ResWriter, Resource};

/// Version-information resource. (`VERSIONINFO`)
///
/// Defines a version-information resource. The resource contains information about the file
/// as its version number, intended operating system, original filename, etc.
///
/// More info: <https://learn.microsoft.com/windows/win32/menurc/versioninfo-resource>
#[derive(Debug, Default)]
pub struct VersionInfo {
    /// Fixed fields of the `VERSIONINFO` resource.
    pub fixed: FixedInfo,
    /// String and variable information block of the `VERSIONINFO` resource.
    pub block: BlockInfo,
}

impl VersionInfo {
    /// Creates a new `VERSIONINFO`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `VERSIONINFO` with the default values retrieved from Cargo's environment variables.
    pub fn from_env() -> Result<Self, ResError> {
        Ok(Self {
            fixed: FixedInfo::from_env()?,
            block: BlockInfo::from_env()?,
        })
    }

    /// Sets the Windows Language Code of the resource.
    ///
    /// Some possible values:
    /// * Language Netural: `0x0000`
    /// * English (`en`): `0x0009`
    /// * English - United States (`en-US`): `0x0409`
    ///
    /// More info about the accepted values:\
    /// <https://learn.microsoft.com/openspecs/windows_protocols/ms-lcid>
    pub fn with_language(mut self, language: u16) -> Self {
        self.block.language = language;
        self
    }
}

impl Resource for VersionInfo {
    fn write(&self, writer: &mut ResWriter) -> Result<(), ResError> {
        writer.line("1 VERSIONINFO");
        self.fixed.write(writer);
        self.block.write(writer);
        Ok(())
    }
}
