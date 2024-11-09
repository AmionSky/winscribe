/// Composable application manifest features.
pub enum Feature {
    /// Sets the process DPI awareness mode.
    DpiAware(DpiMode),
    /// Use Common Control DLL (ComCtl32.dll) version 6.
    ControlsV6,
}

impl Feature {
    pub fn xml(&self) -> String {
        match self {
            Self::DpiAware(mode) => mode.xml(),
            Self::ControlsV6 => include_str!("../../../manifest/controlsv6.xml").to_string(),
        }
    }
}

/// DPI Awareness Mode
///
/// More info: <https://learn.microsoft.com/windows/win32/hidpi>
#[derive(Debug)]
pub enum DpiMode {
    Unaware,
    System,
    PerMonitor,
    PerMonitorV2,
}

impl DpiMode {
    pub fn xml(&self) -> String {
        let (aware, awareness) = match self {
            Self::Unaware => ("false", "unaware"),
            Self::System => ("true", "system"),
            Self::PerMonitor => ("true/pm", "PerMonitor"),
            Self::PerMonitorV2 => ("true", "PerMonitorV2"), // Does `aware` should be "true/pm" or just "true"?
        };

        let mut xml = String::from(include_str!("../../../manifest/dpiaware.xml"));
        replace(&mut xml, "{aware}", aware);
        replace(&mut xml, "{awareness}", awareness);
        xml
    }
}

fn replace(string: &mut String, key: &str, value: &str) {
    let index = string.find(key).unwrap();
    string.replace_range(index..index + key.len(), value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dpimode() {
        // Checks the replace calls inside DpiMode::xml()
        assert!(!DpiMode::System.xml().is_empty());
    }
}
