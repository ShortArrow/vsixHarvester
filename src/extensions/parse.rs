use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ExtensionInfo {
    // Key is target platform (None if not exists)
    // Value is the latest version for the platform
    pub arch_versions: HashMap<Option<String>, String>,
}

pub fn parse(
    response_json: &serde_json::Value,
) -> Result<HashMap<Option<String>, String>, &'static str> {
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
    Ok(arch_versions)
}
pub const LOG3: &str = r#""
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
pub const LOG2: &str = r#"
{
    "results": [
        {
            "extensions": [
                {
                    "versions": [
                        {
                            "version": "0.0.10"
                        }
                    ]
                }
            ]
        }
    ]
}
"#;

pub const LOG1: &str = r#"
{
    "results": [
        {
            "extensions": [
                {
                    "versions": [
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "win32-x64"
                        },
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "linux-arm64"
                        },
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "darwin-x64"
                        },
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "darwin-arm64"
                        },
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "alpine-x64"
                        },
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "win32-arm64"
                        },
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "linux-x64"
                        },
                        {
                            "version": "0.4.2304",
                            "targetPlatform": "linux-armhf"
                        },
                        {
                            "version": "0.4.1731",
                            "targetPlatform": "win32-ia32"
                        },
                        {
                            "version": "0.4.1067"
                        }
                    ]
                }
            ]
        }
    ]
}
"#;
