use std::fs;
use std::path::Path;
use crate::version;

pub async fn download_extension(
    extension: &str,
    destination: &str,
    no_cache: bool,
    proxy: Option<&str>,
    verbose: bool,
    os_arch: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("Progress in extension: {}", extension);
    }

    let parts: Vec<&str> = extension.split('.').collect();
    if parts.len() != 2 {
        eprintln!("Invalid extension identifier: {}", extension);
        return Ok(());
    }
    let publisher = parts[0];
    let extension_name = parts[1];

    // Get latest version
    let (version, _platforms) = version::get_extension_info(publisher, extension_name, proxy, verbose).await?;
    if verbose {
        println!("Latest version of {}: {:?}", extension, version);
    }

    // Create download url
    let target_platform = match os_arch {
        Some("darwin-x64") => "darwin-x64",
        Some("darwin-arm64") => "darwin-arm64",
        Some("win32-x64") => "win32-x64",
        Some("win32-arm64") => "win32-arm64",
        Some("linux-x64") => "linux-x64",
        Some("linux-arm64") => "linux-arm64",
        Some(other) => {
            eprintln!("Unsupported OS architecture: {}", other);
            return Ok(());
        }
        None => {
            eprintln!("OS architecture not specified.");
            return Ok(());
        }
    };

    let download_url = format!(
        "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension_name}/{version}/vspackage?targetPlatform={target_platform}",
        publisher = publisher,
        extension_name = extension_name,
        version = version,
        target_platform = target_platform
    );

    if verbose {
        println!("Download URL: {}", download_url);
    }

    // Make file path
    let file_name = format!("{publisher}.{extension_name}-{version}@{target_platform}.vsix");
    let file_path = format!("{}/{}", destination, file_name);

    // Check file already exists
    if !no_cache && Path::new(&file_path).exists() {
        if verbose {
            println!(
                "Skip download: File is already exists. File Name {}.",
                file_path
            );
        }
        return Ok(());
    }

    // Create http client
    let client_builder = reqwest::Client::builder();
    let client = if let Some(proxy_url) = proxy {
        if verbose {
            println!("Using proxy: {}", proxy_url);
        }
        let proxy = reqwest::Proxy::all(proxy_url)?;
        client_builder.proxy(proxy).build()?
    } else {
        client_builder.build()?
    };

    // Download VSIX file
    if verbose {
        println!("Download form {}", download_url);
    }
    let resp = client.get(&download_url).send().await?;
    if !resp.status().is_success() {
        eprintln!("Fail download of {}", extension);
        return Err(Box::from("Fail download of VSIX"));
    }
    let vsix_content = resp.bytes().await?;

    // Save file
    fs::write(&file_path, &vsix_content)?;
    if verbose {
        println!("Saved in {}", file_path);
    }

    Ok(())
}
