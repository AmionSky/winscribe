//! Application manifest resource. ([`RT_MANIFEST`](crate::manifest::Manifest))

mod features;

pub use features::*;

use crate::{util, ResError, ResWriter, Resource};
use std::path::{Path, PathBuf};

/// Application manifest resource. (`RT_MANIFEST`)
///
/// Use `Manifest::from(...)` to construst it.
///
/// It can be construsted from:
/// * Single or list of [`Feature`](crate::manifest::Feature) to compose a new manifest.
/// * Path to a manifest. (must be a [`PathBuf`](std::path::PathBuf) or [`&Path`](std::path::Path))
/// * Contents of a manifest as a [`String`].
///
/// ### Example:
/// ```
/// # use winscribe::manifest::{Manifest, Feature, DpiMode};
/// # use std::path::Path;
/// // From features:
/// Manifest::from([
///     Feature::DpiAware(DpiMode::PerMonitorV2),
///     Feature::ControlsV6,
/// ]);
/// // From path:
/// Manifest::from(Path::new("my_manifest.xml"));
/// ```
#[derive(Debug)]
pub enum Manifest {
    Internal(String),
    External(PathBuf),
}

impl Resource for Manifest {
    fn write(&self, writer: &mut ResWriter) -> Result<(), ResError> {
        match self {
            Manifest::Internal(xml) => {
                let path = util::out_file("manifest.xml")?;
                util::to_file(&path, xml.as_bytes())?;
                write_manifest(writer, &path)
            }
            Manifest::External(path) => write_manifest(writer, path),
        }
    }
}

fn write_manifest<P: AsRef<Path>>(writer: &mut ResWriter, path: P) -> Result<(), ResError> {
    Ok(writer.line(format!("1 24 \"{}\"", util::escape_path(path)?)))
}

impl From<String> for Manifest {
    fn from(value: String) -> Self {
        Self::Internal(value)
    }
}

impl From<&[Feature]> for Manifest {
    fn from(value: &[Feature]) -> Self {
        let mut buffer = String::with_capacity(1024);

        buffer.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
        buffer.push_str(
            "<assembly xmlns=\"urn:schemas-microsoft-com:asm.v1\" manifestVersion=\"1.0\">\n",
        );

        for feature in value {
            buffer.push_str(&feature.xml());
            buffer.push('\n');
        }

        buffer.push_str("</assembly>");

        Self::Internal(buffer)
    }
}

impl From<Feature> for Manifest {
    fn from(value: Feature) -> Self {
        Self::from([value])
    }
}

impl<const N: usize> From<[Feature; N]> for Manifest {
    fn from(value: [Feature; N]) -> Self {
        Self::from(value.as_slice())
    }
}

impl<const N: usize> From<&[Feature; N]> for Manifest {
    fn from(value: &[Feature; N]) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<PathBuf> for Manifest {
    fn from(value: PathBuf) -> Self {
        Self::External(value)
    }
}

impl From<&Path> for Manifest {
    fn from(value: &Path) -> Self {
        Self::External(value.to_path_buf())
    }
}
