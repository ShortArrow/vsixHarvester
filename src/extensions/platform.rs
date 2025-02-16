use crate::extensions::info;

use super::version;

pub fn get_current() -> String {
    // # Provide the following ARCH values
    // - x86
    // - x86_64
    // - arm
    // - aarch64
    // - loongarch64
    // - m68k
    // - csky
    // - mips
    // - mips64
    // - powerpc
    // - powerpc64
    // - riscv64
    // - s390x
    // - sparc64
    let arch = std::env::consts::ARCH.to_string();
    let modified_arch = match arch.as_str() {
        "x86_64" => "x64".to_string(),
        "arm" => "arm64".to_string(),
        _ => arch,
    };
    // # Provide the following OS values
    // - linux
    // - macos
    // - ios
    // - freebsd
    // - dragonfly
    // - netbsd
    // - openbsd
    // - solaris
    // - android
    // - windows
    let os = std::env::consts::OS.to_string();
    let modified_os = match os.as_str() {
        "macos" => "darwin".to_string(),
        "windows" => "win32".to_string(),
        _ => os,
    };
    // # To be supported
    // - `win32-x64`
    // - `win32-arm64`
    // - `darwin-x64`
    // - `darwin-arm64`
    // - `linux-x64`
    // - `linux-arm64`
    // - `alpine-x64`
    // - `alpine-arm64`
    // - `win32-ia32`
    // - `linux-armhf`
    // - `web`
    format!("{modified_os}-{modified_arch}")
}

pub fn decide_target(
    specified: Option<&str>,
    current: String,
    info: info::ExtensionInfo,
) -> Option<String> {
    if let Some(specified) = specified {
        if info
            .arch_versions
            .contains_key(&Some(specified.to_string()))
        {
            Some(specified.to_owned())
        } else {
            eprintln!(
                "Unsupported OS architecture: {specified} is not supported for {:?}",
                info.arch_versions
            );
            None
        }
    } else if info.arch_versions.contains_key(&Some(current.clone())) {
        let is_none_supported = info.arch_versions.contains_key(&None);
        if !is_none_supported {
            return Some(current);
        }
        // Check if the current version is supported and if there is a version for None
        if let Some(none_version_str) = info.arch_versions.get(&None) {
            let current_version =
                version::parse(info.arch_versions.get(&Some(current.clone())).unwrap());

            let none_version = version::parse(none_version_str);
            if current_version > none_version {
                Some(current)
            } else {
                None
            }
        } else {
            Some(current)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::info::ExtensionInfo;
    use rstest::rstest;
    use std::collections::HashMap;

    fn pattern1() -> ExtensionInfo {
        ExtensionInfo {
            arch_versions: {
                let mut map = HashMap::new();
                map.insert(None, "10.1.0".to_string());
                map.insert(Some("win32-x64".to_string()), "10.1.0".to_string());
                map.insert(Some("win32-arm64".to_string()), "10.1.0".to_string());
                map.insert(Some("linux-x64".to_string()), "10.1.0".to_string());
                map.insert(Some("linux-arm64".to_string()), "10.1.3".to_string());
                map.insert(Some("web".to_string()), "10.1.0".to_string());
                map.insert(Some("aarch64".to_string()), "10.1.0".to_string());
                map
            },
        }
    }

    #[rstest]
    #[case(None, Some("test"))]
    #[case(Some("web"), Some("web"))]
    #[case(Some("win32-x64"), Some("win32-x64"))]
    #[case(Some("linux-x64"), Some("linux-x64"))]
    fn test_when_current_is_supported(
        #[case] expected: Option<&str>,
        #[case] target: Option<&str>,
    ) {
        let arch_versions = pattern1().arch_versions;
        let current = "linux-x64".to_string();
        let info = ExtensionInfo { arch_versions };
        assert_eq!(
            expected,
            decide_target(target, current, info.clone()).as_deref()
        );
    }

    #[test]
    fn test_when_current_is_not_supported() {
        let current = "linux-x64".to_string();
        let mut arch_versions = HashMap::new();
        arch_versions.insert(Some("x64".to_string()), "ver".to_string());
        arch_versions.insert(Some("x86".to_string()), "ver".to_string());
        let info = ExtensionInfo { arch_versions };
        assert_eq!(None, decide_target(None, current.clone(), info.clone()));
        assert_eq!(
            Some("x64".to_string()),
            decide_target(Some("x64"), current.clone(), info.clone())
        );
        assert_eq!(None, decide_target(Some(&current), current.clone(), info));
    }

    #[test]
    fn test_when_no_supported_architectures() {
        let current = "linux-x64".to_string();
        let info = ExtensionInfo {
            arch_versions: HashMap::new(),
        };
        assert_eq!(None, decide_target(None, current.clone(), info.clone()));
        assert_eq!(None, decide_target(Some("x64"), current, info));
    }
}
