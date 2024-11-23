use serde_json::json;

pub async fn get_extension_version(
    publisher: &str,
    extension_name: &str,
    proxy: Option<&str>,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
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
        .ok_or("Failed get extension version")?
        .to_string();

    Ok(version)
}
