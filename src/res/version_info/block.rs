use crate::{util, ResError, ResWriter};
use std::collections::HashMap;

/// String and variable information block of the `VERSIONINFO` resource.
#[derive(Debug)]
pub struct BlockInfo {
    /// Windows Language Code
    ///
    /// More info: <https://learn.microsoft.com/openspecs/windows_protocols/ms-lcid>
    pub language: u16,

    /// Set values of the `StringFileInfo` block.
    pub strings: HashMap<StringInfo, String>,
}

impl BlockInfo {
    pub fn new<T: Into<HashMap<StringInfo, String>>>(language: u16, strings: T) -> Self {
        Self {
            language,
            strings: strings.into(),
        }
    }

    /// Create from environment variables set by cargo.
    pub fn from_env() -> Result<Self, ResError> {
        let name = util::env_var("CARGO_PKG_NAME")?;
        let description = util::env_var("CARGO_PKG_DESCRIPTION")?;
        let version_str = util::env_var("CARGO_PKG_VERSION")?;

        Ok(Self {
            language: 0, // Language Natural
            strings: HashMap::from([
                (StringInfo::FileDescription, description),
                (StringInfo::FileVersion, version_str.clone()),
                (StringInfo::InternalName, name.clone()),
                (StringInfo::ProductName, name),
                (StringInfo::ProductVersion, version_str),
            ]),
        })
    }

    pub(super) fn write(&self, writer: &mut ResWriter) {
        const CHARSET: u16 = 1200; // Unicode

        writer.begin();
        writer.block("StringFileInfo");
        writer.block(format!("{:04X}{:04X}", self.language, CHARSET));

        for (key, value) in &self.strings {
            writer.value_str(key.as_str(), value);
        }

        writer.end();
        writer.end();

        // Note: Have not found much documentation about this block.
        writer.block("VarFileInfo");
        writer.value_raw(
            "Translation",
            format!("{:#06X}, {:#06X}", self.language, CHARSET),
        );
        writer.end();

        writer.end();
    }
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self::new(0, HashMap::new())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum StringInfo {
    /// Additional information that should be displayed for diagnostic purposes.
    Comments,
    /// Company that produced the file.
    CompanyName,
    /// File description to be presented to users. This string may be displayed in a list box when
    /// the user is choosing files to install.
    FileDescription,
    /// Version number of the file.
    FileVersion,
    /// Internal name of the file, if one exists. If the file has no internal name, this string
    /// should be the original filename, without extension.
    InternalName,
    /// Copyright notices that apply to the file. This should include the full text of all notices,
    /// legal symbols, copyright dates, and so on.
    LegalCopyright,
    /// Trademarks and registered trademarks that apply to the file. This should include the full
    /// text of all notices, legal symbols, trademark numbers, and so on.
    LegalTrademarks,
    /// Original name of the file, not including a path. This information enables an application to
    /// determine whether a file has been renamed by a user. The format of the name depends on the
    /// file system for which the file was created.
    OriginalFilename,
    /// Information about a private version of the file. This string should be present only if
    /// VS_FF_PRIVATEBUILD is specified in the fileflags parameter of the root block.
    PrivateBuild,
    /// Name of the product with which the file is distributed.
    ProductName,
    /// Version of the product with which the file is distributed.
    ProductVersion,
    /// Text that specifies how this version of the file differs from the standard version.
    /// This string should be present only if VS_FF_SPECIALBUILD is specified in the fileflags
    /// parameter of the root block.
    SpecialBuild,
}

impl StringInfo {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Comments => "Comments",
            Self::CompanyName => "CompanyName",
            Self::FileDescription => "FileDescription",
            Self::FileVersion => "FileVersion",
            Self::InternalName => "InternalName",
            Self::LegalCopyright => "LegalCopyright",
            Self::LegalTrademarks => "LegalTrademarks",
            Self::OriginalFilename => "OriginalFilename",
            Self::PrivateBuild => "PrivateBuild",
            Self::ProductName => "ProductName",
            Self::ProductVersion => "ProductVersion",
            Self::SpecialBuild => "SpecialBuild",
        }
    }
}
