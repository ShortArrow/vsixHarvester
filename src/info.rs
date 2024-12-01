use serde_json::json;
use log::{debug, info};

pub async fn get(
    publisher: &str,
    extension_name: &str,
    proxy: Option<&str>,
    verbose: bool,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    let api_url = "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";

    let payload = json!({
        "filters": [{
            "criteria": [
                {"filterType": 7, "value": format!("{}.{}", publisher, extension_name)}
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
        println!(
            "Sending query for Marketplace API: {}.{}",
            publisher, extension_name
        );
    }
    let resp = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json;api-version=3.0-preview.1")
        .header("User-Agent", "Offline VSIX/1.0")
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        eprintln!("Failed query for Marketplace API");
        return Err(Box::from("Failed query for Marketplace API"));
    }

    let resp_json: serde_json::Value = resp.json().await?;

    // Extract version
    let version = resp_json["results"][0]["extensions"][0]["versions"][0]["version"]
        .as_str()
        .ok_or("Failed to get extension version")?
        .to_string();
    debug!("Response debug: {:?}", version);
    info!("Response info: {:?}", version);
    println!("{:?}", version);

    // Extract supported architectures
    let architectures = resp_json["results"][0]["extensions"][0]["versions"]
        .as_array()
        .ok_or("Failed to get versions array")?
        .iter()
        .filter_map(|v| v["targetPlatform"].as_str().map(|s| s.to_string()))
        .collect();
    debug!("Response debug: {:#?}", architectures);
    info!("Response info: {:#?}", architectures);
    println!("{:?}", architectures);

    Ok((version, architectures))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_extension_info() {
        let (version, architectures) = get("rust-lang", "rust-analyzer", None, false)
            .await
            .unwrap();
        let firstversion = version.chars().next().unwrap();
        let secondversion = version.chars().nth(2).unwrap();
        let thirdversion = version.chars().nth(4).unwrap();
        let is_digit_first = firstversion.is_digit(10);
        let is_digit_second = secondversion.is_digit(10);
        let is_digit_third = thirdversion.is_digit(10);
        assert_eq!(is_digit_first, true);
        assert_eq!(is_digit_second, true);
        assert_eq!(is_digit_third, true);
        // expect ["win32-arm64", "darwin-x64", "win32-x64", "linux-armhf", "linux-x64", "linux-arm64", "alpine-x64", "darwin-arm64", "win32-ia32"]
        assert!(architectures.contains(&"win32-arm64".to_string()));
        assert!(architectures.contains(&"darwin-x64".to_string()));
        assert!(architectures.contains(&"win32-x64".to_string()));
        assert!(architectures.contains(&"linux-armhf".to_string()));
        assert!(architectures.contains(&"linux-x64".to_string()));
        assert!(architectures.contains(&"linux-arm64".to_string()));
        assert!(architectures.contains(&"alpine-x64".to_string()));
        assert!(architectures.contains(&"darwin-arm64".to_string()));
        assert!(architectures.contains(&"win32-ia32".to_string()));
        
        assert_eq!(architectures.len(), 9);
    }
}
