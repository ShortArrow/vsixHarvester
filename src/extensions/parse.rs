use crate::extensions::info::ExtensionInfo;
use std::collections::HashMap;

pub fn parse(response_json: &serde_json::Value) -> Result<ExtensionInfo, &'static str> {
    let versions_array = response_json["results"][0]["extensions"][0]["versions"]
        .as_array()
        .ok_or("Failed to get versions array")?;

    // Restrucuturing the versions array into a dictionary
    // To be each architecuture has the latest version
    let mut arch_versions: HashMap<Option<String>, String> = HashMap::new();
    for v in versions_array {
        if let Some(version_str) = v["version"].as_str() {
            let arch = v
                .get("targetPlatform")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            if !arch_versions.contains_key(&arch) {
                arch_versions.insert(arch, version_str.to_string());
            }
        }
    }
    Ok(ExtensionInfo { arch_versions })
}

#[cfg(test)]
mod tests {
    use super::*;
    const LOG3: &str = r#"
{
    "results": [
        {
            "extensions": [
                {
                    "versions": [
                        {
                            "version": "1.11.3"
                        },
                        {
                            "version": "1.11.0",
                            "targetPlatform": "linux-x64"
                        }
                    ]
                }
            ]
        }
    ]
}
"#;
    const LOG2: &str = r#"
{
    "results": [
        {
            "extensions": [
                {
                    "versions": [
                        {"version": "0.0.10"}
                    ]
                }
            ]
        }
    ]
}
"#;

    const LOG1: &str = r#"
{
    "results": [
        {
            "extensions": [
                {
                    "versions": [
                        {"version": "0.4.2304", "targetPlatform": "win32-x64"},
                        {"version": "0.4.2304", "targetPlatform": "linux-arm64"},
                        {"version": "0.4.2304", "targetPlatform": "darwin-x64"},
                        {"version": "0.4.2304", "targetPlatform": "darwin-arm64"},
                        {"version": "0.4.2304", "targetPlatform": "alpine-x64"},
                        {"version": "0.4.2304", "targetPlatform": "win32-arm64"},
                        {"version": "0.4.2304", "targetPlatform": "linux-x64"},
                        {"version": "0.4.2304", "targetPlatform": "linux-armhf"},
                        {"version": "0.4.1731", "targetPlatform": "win32-ia32"},
                        {"version": "0.4.1067"}
                    ]
                }
            ]
        }
    ]
}
"#;

    #[test]
    fn test_parse_parameterized_log1() {
        let response_json: serde_json::Value = serde_json::from_str(LOG1).unwrap();
        let extension_info = parse(&response_json).unwrap();
        let arch_versions = extension_info.arch_versions;
        let expected = vec![
            (Some("win32-x64".to_string()), "0.4.2304".to_string()),
            (Some("linux-arm64".to_string()), "0.4.2304".to_string()),
            (Some("darwin-x64".to_string()), "0.4.2304".to_string()),
            (Some("darwin-arm64".to_string()), "0.4.2304".to_string()),
            (Some("alpine-x64".to_string()), "0.4.2304".to_string()),
            (Some("win32-arm64".to_string()), "0.4.2304".to_string()),
            (Some("linux-x64".to_string()), "0.4.2304".to_string()),
            (Some("linux-armhf".to_string()), "0.4.2304".to_string()),
            (Some("win32-ia32".to_string()), "0.4.1731".to_string()),
            (None, "0.4.1067".to_string()),
        ];
        for (platform, version) in expected {
            assert_eq!(arch_versions.get(&platform), Some(&version));
        }
    }

    #[test]
    fn test_parse_parameterized_log2() {
        let response_json: serde_json::Value = serde_json::from_str(LOG2).unwrap();
        let extension_info = parse(&response_json).unwrap();
        let arch_versions = extension_info.arch_versions;
        let expected = vec![(None, "0.0.10".to_string())];
        for (platform, version) in expected {
            assert_eq!(arch_versions.get(&platform), Some(&version));
        }
    }

    #[test]
    fn test_parse_parameterized_log3() {
        let response_json: serde_json::Value = serde_json::from_str(LOG3).unwrap();
        let extension_info = parse(&response_json).unwrap();
        let arch_versions = extension_info.arch_versions;
        let expected = vec![
            (None, "1.11.3".to_string()),
            (Some("linux-x64".to_string()), "1.11.0".to_string()),
        ];
        for (platform, version) in expected {
            assert_eq!(arch_versions.get(&platform), Some(&version));
        }
    }
}
