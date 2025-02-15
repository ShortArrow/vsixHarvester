use crate::extensions::info;

pub fn decide_target(os_arch: Option<&str>, info: info::ExtensionInfo) -> Option<&str> {
    let current = std::env::consts::ARCH;
    let supporteds = info.architectures.clone();
    if supporteds.is_empty() {
        return None;
    }
    match os_arch {
        Some(arch) if supporteds.contains(&arch.to_string()) => Some(arch),
        Some(arch) => {
            eprintln!("Unsupported OS architecture: {arch} is not supported for {supporteds:?}");
            None
        }
        None if supporteds.contains(&current.to_string()) => Some(current),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_when_current_is_supported() {
        let current = std::env::consts::ARCH;
        let info = info::ExtensionInfo {
            architectures: vec![current.to_string(), "x64".to_string(), "x86".to_string()],
            ..Default::default()
        };
        assert_eq!(Some("x64"), decide_target(Some("x64"), info.clone()));
        assert_eq!(Some("x86"), decide_target(Some("x86"), info.clone()));
        assert_eq!(None, decide_target(Some("tekito"), info.clone()));
        assert_eq!(Some(current), decide_target(None, info));
    }
    #[test]
    fn test_when_current_is_not_supported() {
        let info = info::ExtensionInfo {
            architectures: vec!["x64".to_string(), "x86".to_string()],
            ..Default::default()
        };
        let current = std::env::consts::ARCH;
        assert_eq!(None, decide_target(None, info.clone()));
        assert_eq!(Some("x64"), decide_target(Some("x64"), info.clone()));
        assert_eq!(None, decide_target(Some(current), info));
    }
    #[test]
    fn test_when_no_supported_architectures() {
        let info = info::ExtensionInfo {
            architectures: vec![],
            ..Default::default()
        };
        assert_eq!(None, decide_target(None, info.clone()));
        assert_eq!(None, decide_target(Some("x64"), info));
    }
}
