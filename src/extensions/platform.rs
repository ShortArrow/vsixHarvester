use crate::extensions::info;

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

pub fn decide_target(specified: Option<&str>, info: info::ExtensionInfo) -> Option<String> {
    let current = get_current();
    let supporteds = info.architectures.clone();
    if supporteds.is_empty() {
        return None;
    }
    match specified {
        Some(specified) if supporteds.contains(&specified.to_string()) => Some(specified.to_owned()),
        Some(specified) => {
            eprintln!(
                "Unsupported OS architecture: {specified} is not supported for {supporteds:?}"
            );
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
        let current = get_current();
        let info = info::ExtensionInfo {
            architectures: vec![current.clone(), "x64".to_string(), "x86".to_string()],
            ..Default::default()
        };
        assert_eq!(Some("x64".to_string()), decide_target(Some("x64"), info.clone()));
        assert_eq!(Some("x86".to_string()), decide_target(Some("x86"), info.clone()));
        assert_eq!(None, decide_target(Some("tekito"), info.clone()));
        assert_eq!(Some(current), decide_target(None, info));
    }
    #[test]
    fn test_when_current_is_not_supported() {
        let info = info::ExtensionInfo {
            architectures: vec!["x64".to_string(), "x86".to_string()],
            ..Default::default()
        };
        let current = &get_current();
        assert_eq!(None, decide_target(None, info.clone()));
        assert_eq!(Some("x64".to_string()), decide_target(Some("x64"), info.clone()));
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
