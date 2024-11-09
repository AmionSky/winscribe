use crate::{util, ResError, ResWriter};

const FILE_FLAGS_MASK: u32 = 0x3F; // VS_FFI_FILEFLAGSMASK
const OS_WIN_NT_32: u32 = 0x40004; // Windows NT 32-bit

/// Fixed fields of the `VERSIONINFO` resource.
#[derive(Debug)]
pub struct FixedInfo {
    /// `FILEVERSION` - Binary version number for the file.
    pub version: Version,
    /// `PRODUCTVERSION` - Binary version number for the product.
    pub product_version: Version,
    /// `FILEFLAGSMASK` - Indicates which bits in the FILEFLAGS statement are valid.
    pub flags_mask: u32,
    /// `FILEFLAGS` - Attributes of the file.
    pub flags: FileFlags,
    /// `FILEOS` - Operating system for which this file was designed.
    pub os: u32,
    /// `FILETYPE` - General type of file.
    pub file_type: FileType,
    /// `FILESUBTYPE` - Function of the file. The subtype parameter is zero unless the
    /// filetype parameter in the `FILETYPE` statement is `DRV`, `FONT`, or `VXD`.
    pub sub_type: u32,
}

impl FixedInfo {
    /// Create from environment variables set by cargo.
    pub fn from_env() -> Result<Self, ResError> {
        let version = Version::from_env()?;

        Ok(FixedInfo {
            version: version.clone(),
            product_version: version,
            flags_mask: FILE_FLAGS_MASK,
            flags: FileFlags::from_env()?,
            os: OS_WIN_NT_32,
            file_type: FileType::App,
            sub_type: 0,
        })
    }

    pub(super) fn write(&self, writer: &mut ResWriter) {
        writer.line(format_ver("FILEVERSION", &self.version));
        writer.line(format_ver("PRODUCTVERSION", &self.product_version));
        writer.line(format!("FILEFLAGSMASK {:#X}", self.flags_mask));
        writer.line(format!("FILEFLAGS {:#X}", self.flags.val()));
        writer.line(format!("FILEOS {:#X}", self.os));
        writer.line(format!("FILETYPE {:#X}", self.file_type as u32));
        writer.line(format!("FILESUBTYPE {:#X}", self.sub_type));
    }
}

impl Default for FixedInfo {
    fn default() -> Self {
        Self {
            version: Version::zero(),
            product_version: Version::zero(),
            flags_mask: FILE_FLAGS_MASK,
            flags: FileFlags::None,
            os: 0, // Unknown OS
            file_type: FileType::Unknown,
            sub_type: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub revision: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16, revision: u16) -> Self {
        Self {
            major,
            minor,
            patch,
            revision,
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Parses the `CARGO_PKG_VERSION_*` environment variables.
    ///
    /// Revision number is set to 0.
    pub fn from_env() -> Result<Self, ResError> {
        Ok(Self::new(
            util::env_var("CARGO_PKG_VERSION_MAJOR")?
                .parse()
                .map_err(|_| ResError::Custom("Failed to parse version! (major)"))?,
            util::env_var("CARGO_PKG_VERSION_MINOR")?
                .parse()
                .map_err(|_| ResError::Custom("Failed to parse version! (minor)"))?,
            util::env_var("CARGO_PKG_VERSION_PATCH")?
                .parse()
                .map_err(|_| ResError::Custom("Failed to parse version! (patch)"))?,
            0,
        ))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.revision
        )
    }
}

#[derive(Debug)]
pub enum FileFlags {
    /// No flags are set.
    None,
    /// File contains debugging information or is compiled with debugging features enabled.
    Debug,
    /// File has been modified and is not identical to the original shipping file of the same
    /// version number.
    Patched,
    /// File is a development version, not a commercially released product.
    Prerelease,
    /// File was not built using standard release procedures. If this value is given, the
    /// `StringFileInfo` block must contain a `PrivateBuild` string.
    PrivateBuild,
    /// File was built by the original company using standard release procedures but is a
    /// variation of the standard file of the same version number. If this value is given,
    /// the `StringFileInfo` block block must contain a `SpecialBuild` string.
    SpecialBuild,
    /// The combination of multiple flags.
    Combined(u32),
}

impl FileFlags {
    /// Gets the value.
    pub fn val(&self) -> u32 {
        match *self {
            Self::None => 0x0,
            Self::Debug => 0x1,
            Self::Patched => 0x4,
            Self::Prerelease => 0x2,
            Self::PrivateBuild => 0x8,
            Self::SpecialBuild => 0x20,
            Self::Combined(v) => v,
        }
    }

    /// Sets the flags based on environment variables set by cargo.
    pub fn from_env() -> Result<Self, ResError> {
        let mut flags = Self::None;

        if util::env_var_os("DEBUG")? == "true" {
            flags |= Self::Debug;
        }
        if !util::env_var_os("CARGO_PKG_VERSION_PRE")?.is_empty() {
            flags |= Self::Prerelease;
        }

        Ok(flags)
    }
}

impl std::ops::BitOr for FileFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Combined(self.val() | rhs.val())
    }
}

impl std::ops::BitOrAssign for FileFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self::Combined(self.val() | rhs.val())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    /// File type is unknown.
    Unknown = 0x0,
    /// The file is an application.
    App = 0x1,
    /// The file is a dynamic link library.
    DLL = 0x2,
    /// The file is a device driver.
    DRV = 0x3,
    /// The file is a font.
    Font = 0x4,
    /// The file is a virtual device.
    VXD = 0x5,
    /// The file is a static link library.
    StaticLib = 0x7,
}

fn format_ver(field: &str, version: &Version) -> String {
    format!(
        "{} {},{},{},{}",
        field, version.major, version.minor, version.patch, version.revision
    )
}
