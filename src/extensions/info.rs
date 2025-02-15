use crate::extensions::url::query_url;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ExtensionName {
    pub name: String,
    pub publisher: String,
}

pub fn validate_extension_name(name: &str) -> bool {
    let parts: Vec<&str> = name.split('.').collect();
    parts.len() == 2
}

pub fn parse_extension_name(name: &str) -> ExtensionName {
    if !validate_extension_name(name) {
        eprintln!("Invalid extension format: {name}");
        panic!("Extension name must be in the format 'publisher.name'");
    }
    let parts: Vec<&str> = name.split('.').collect();
    let publisher = parts[0];
    let name = parts[1];
    ExtensionName {
        name: name.to_string(),
        publisher: publisher.to_string(),
    }
}

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

pub async fn get(
    publisher: &str,
    extension_name: &str,
    proxy: Option<&str>,
    verbose: bool,
) -> Result<ExtensionInfo, Box<dyn std::error::Error>> {
    let payload = json!({
        "filters": [{
            "criteria": [
                {"filterType": 7, "value": format!("{publisher}.{extension_name}")}
            ]
        }],
        "flags": 914
    });

    // Create http client
    let client_builder = reqwest::Client::builder();
    let client = if let Some(proxy_url) = proxy {
        if verbose {
            println!("Using proxy for API request: {}", proxy_url);
        }
        let proxy = reqwest::Proxy::all(proxy_url)?;
        client_builder.proxy(proxy).build()?
    } else {
        client_builder.build()?
    };

    // Send POST request
    if verbose {
        println!("Sending query for Marketplace API: {publisher}.{extension_name}");
    }
    let response = client
        .post(&query_url())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json;api-version=3.0-preview.1")
        .header("User-Agent", "Offline VSIX/1.0")
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("Failed query for Marketplace API");
        return Err(Box::from("Failed query for Marketplace API"));
    }

    let response_json: serde_json::Value = response.json().await?;
    let arch_versions: HashMap<Option<String>, String> = parse(&response_json)?;

    Ok(ExtensionInfo { arch_versions })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_extension_info() {
        let extension_info = get("rust-lang", "rust-analyzer", None, false)
            .await
            .unwrap();

        // Check if the expected platforms exist
        let expected_archs = vec![
            Some("win32-arm64".to_string()),
            Some("darwin-x64".to_string()),
            Some("win32-x64".to_string()),
            Some("linux-armhf".to_string()),
            Some("linux-x64".to_string()),
            Some("linux-arm64".to_string()),
            Some("alpine-x64".to_string()),
            Some("darwin-arm64".to_string()),
            Some("win32-ia32".to_string()),
        ];
        for arch in expected_archs {
            assert!(
                extension_info.arch_versions.contains_key(&arch),
                "Missing arch: {:?}",
                arch
            );
        }
        // Check that the first character of each version string is a digit
        for v in extension_info.arch_versions.values() {
            let first = v.chars().next().unwrap();
            assert!(
                first.is_digit(10),
                "Version string should start with a digit: {}",
                v
            );
        }
    }
}
