//! A build script to create, compile and include Windows resource and manifest
//! files in your executables.
//!
//! ## Usage:
//!
//! Inside `build.rs`:
//! ```no_run
//! // Check if the target OS is Windows
//! if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
//!     use winscribe::ResBuilder;
//!     use winscribe::manifest::{Manifest, Feature, DpiMode};
//!
//!     // Creates a resource with some info inferred from cargo environment variables
//!     // Ex: version, name, description
//!     // Same as calling: `ResBuilder::new().push(VersionInfo::from_env()?)`
//!     ResBuilder::from_env()
//!         .expect("Failed to create resource from environment!")
//!         // Creates a manifest with DPI awareness set to Per-Monitor V2
//!         .push(Manifest::from(Feature::DpiAware(DpiMode::PerMonitorV2)))
//!         // Compiles and links the resource to the binary
//!         .compile()
//!         .expect("Failed to compile the resource file!");
//! }
//! ```

mod compiler;
mod error;
mod res;
mod util;
mod writer;

pub use error::ResError;
pub use res::*;

use std::path::Path;
use writer::ResWriter;

pub trait Resource {
    fn write(&self, writer: &mut ResWriter) -> Result<(), ResError>;
}

/// Windows Resource file builder.
#[derive(Default)]
pub struct ResBuilder {
    resources: Vec<Box<dyn Resource>>,
}

impl ResBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a resource with some info inferred from cargo environment variables.
    ///
    /// Same as calling: `ResBuilder::new().push(VersionInfo::from_env()?)`
    pub fn from_env() -> Result<Self, ResError> {
        Ok(Self::new().push(version_info::VersionInfo::from_env()?))
    }

    /// Adds a new resource.
    pub fn push<T: 'static + Resource>(mut self, resource: T) -> Self {
        self.resources.push(Box::new(resource));
        self
    }

    /// Saves the resource as file.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ResError> {
        let mut writer = ResWriter::new();

        for res in &self.resources {
            writer.new_line(); // Put one empty line between definitions
            res.write(&mut writer)?;
        }

        util::to_file(path, writer.as_bytes())?;
        Ok(())
    }

    /// Compiles and links the resource to the binary being built.
    pub fn compile(&self) -> Result<(), ResError> {
        // Write the resource's .rc file to disk
        let rc_path = util::out_file("resource.rc")?;
        self.save(&rc_path)?;

        // Compile the .rc file into a .res file
        let res_path = util::out_file("resource.res")?;
        compiler::compile(rc_path, &res_path)?;

        // Link the .res file to the binary
        println!("cargo:rustc-link-arg-bins={}", res_path.display());
        Ok(())
    }
}
