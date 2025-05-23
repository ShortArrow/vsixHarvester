use crate::extensions::info;
use crate::extensions::info::parse_extension_name;
use crate::extensions::platform;
use crate::extensions::url;
use std::fs;
use std::io::Read; // For GzDecoder
use flate2::read::GzDecoder; // For GzDecoder
use std::path::Path;

fn name(
    target_platform: Option<String>,
    publisher: &str,
    extension_name: &str,
    version: &str,
) -> String {
    match target_platform {
        Some(target_platform) => {
            format!("{publisher}.{extension_name}-{version}@{target_platform}.vsix")
        }
        None => format!("{publisher}.{extension_name}-{version}.vsix"),
    }
}

pub async fn download(
    extension: &str,
    destination: &str,
    force: bool,
    proxy: Option<&str>,
    verbose: bool,
    os_arch: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("Progress in extension: {extension}");
    }

    let parsed_extension_name = parse_extension_name(extension);
    let publisher = &parsed_extension_name.publisher;
    let extension_name = &parsed_extension_name.name;

    // Get latest version
    let extension_info = info::get(publisher, extension_name, proxy, verbose).await?;
    let versions = &extension_info.arch_versions.clone();
    if verbose {
        println!("Latest version of {extension}: {versions:?}");
    }

    // Create download url
    let current = platform::get_current();
    let target_platform = platform::decide_target(os_arch, current, extension_info.clone());
    let latest_version = extension_info.arch_versions.get(&target_platform.clone());
    let download_url = url::for_download(
        publisher,
        extension_name,
        latest_version.clone().unwrap(),
        target_platform.clone(),
    );
    if verbose {
        println!("Download URL: {download_url:?}");
    }

    // Make file path
    let file_name = name(
        target_platform,
        publisher,
        extension_name,
        latest_version.unwrap(),
    );
    let file_path = format!("{destination}/{file_name}");

    // Check if the file already exists
    if !force && Path::new(&file_path).exists() {
        if verbose {
            println!("Skip download: File is already exists. File Name {file_path}.");
        }
        return Ok(());
    }

    // Create http client
    let client_builder = reqwest::Client::builder();
    let client = if let Some(proxy_url) = proxy {
        if verbose {
            println!("Using proxy: {proxy_url}");
        }
        let proxy = reqwest::Proxy::all(proxy_url)?;
        client_builder.proxy(proxy).build()?
    } else {
        client_builder.build()?
    };

    // Download VSIX file
    if verbose {
        println!("Download from {download_url}");
    }
    let resp = client.get(&download_url)
        .header(reqwest::header::ACCEPT_ENCODING, "gzip")
        .send()
        .await?;
    if !resp.status().is_success() {
        eprintln!("Fail download of {extension}");
        return Err(Box::from("Fail download of VSIX"));
    }
    let compressed_bytes = resp.bytes().await?;
    let mut decoder = GzDecoder::new(&compressed_bytes[..]);
    let mut vsix_content = Vec::new();
    if let Err(e) = decoder.read_to_end(&mut vsix_content) {
        eprintln!("Failed to decompress gzip data for {extension}: {e}");
        return Err(Box::new(e));
    }

    // Save file
    fs::write(&file_path, &vsix_content)?;
    if verbose {
        println!("Saved in {file_path}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_with_platform() {
        let result = name(Some("win32".to_string()), "microsoft", "vscode", "1.0.0");
        assert_eq!(result, "microsoft.vscode-1.0.0@win32.vsix");
    }

    #[test]
    fn test_name_without_platform() {
        let result = name(None, "microsoft", "vscode", "1.0.0");
        assert_eq!(result, "microsoft.vscode-1.0.0.vsix");
    }
}
