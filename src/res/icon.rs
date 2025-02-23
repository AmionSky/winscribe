//! Bitmap icon resource. ([`ICON`](crate::icon::Icon))

use crate::{ResError, ResWriter, Resource, util};
use std::path::PathBuf;

/// Bitmap icon resource. (`ICON`)
///
/// More info: <https://learn.microsoft.com/windows/win32/menurc/icon-resource>
pub struct Icon {
    id: String,
    path: PathBuf,
}

impl Icon {
    /// Creates a new icon resource.
    ///
    /// * `id`: A unique name or a 16-bit unsigned integer.
    /// * `path`: Path to the `.ico` file.
    pub fn new<N, P>(id: N, path: P) -> Self
    where
        N: ToString,
        P: Into<PathBuf>,
    {
        Self {
            id: id.to_string(),
            path: path.into(),
        }
    }

    /// Default application icon. (`IDI_APPLICATION`)
    pub fn app<P: Into<PathBuf>>(path: P) -> Self {
        Self::new(32512, path)
    }
}

impl Resource for Icon {
    fn write(&self, writer: &mut ResWriter) -> Result<(), ResError> {
        writer.line(format!(
            "{} ICON \"{}\"",
            self.id,
            util::escape_path(&self.path)?
        ));
        Ok(())
    }
}
