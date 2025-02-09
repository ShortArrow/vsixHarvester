use crate::info;
use std::fs;
use std::path::Path;

fn get_download_url(
    publisher: &str,
    extension_name: &str,
    version: &str,
    target_platform: Option<&str>,
) -> String {
    match target_platform {
        Some(target_platform) => format!(
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension_name}/{version}/vspackage?targetPlatform={target_platform}",
            ),
        None => format!(
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension_name}/{version}/vspackage",
            ),
    }
}

fn get_target_platform(os_arch: Option<&str>, info: info::ExtensionInfo) -> Option<&str> {
    let current = std::env::consts::ARCH;
    let supporteds = info.architectures.clone();
    if supporteds.iter().count() == 0 {
        return None;
    }
    let specified = match os_arch {
        Some(arch) if supporteds.contains(&arch.to_string()) => Some(arch),
        Some(arch) => {
            eprintln!("Unsupported OS architecture: {arch}");
            None
        }
        None => None,
    };
    if specified.is_some() {
        return specified;
    }
    if supporteds.contains(&current.to_string()) {
        Some(current)
    } else {
        None
    }
}

pub async fn download(
    extension: &str,
    destination: &str,
    no_cache: bool,
    proxy: Option<&str>,
    verbose: bool,
    os_arch: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("Progress in extension: {extension}");
    }

    let parts: Vec<&str> = extension.split('.').collect();
    if parts.len() != 2 {
        eprintln!("Invalid extension identifier: {extension}");
        return Ok(());
    }
    let publisher = parts[0];
    let extension_name = parts[1];

    // Get latest version
    let extension_info = info::get(publisher, extension_name, proxy, verbose).await?;
    let version = &extension_info.version.clone();
    if verbose {
        println!("Latest version of {extension}: {version:?}");
    }

    // Create download url
    let target_platform = get_target_platform(os_arch, extension_info);
    let download_url = get_download_url(publisher, extension_name, version, target_platform);
    if verbose {
        println!("Download URL: {download_url:?}");
    }

    // Make file path
    let file_name = match target_platform {
        Some(target_platform) => {
            format!("{publisher}.{extension_name}-{version}@{target_platform}.vsix")
        }
        None => format!("{publisher}.{extension_name}-{version}.vsix"),
    };
    let file_path = format!("{destination}/{file_name}");

    // Check if the file already exists
    if !no_cache && Path::new(&file_path).exists() {
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
    let resp = client.get(&download_url).send().await?;
    if !resp.status().is_success() {
        eprintln!("Fail download of {extension}");
        return Err(Box::from("Fail download of VSIX"));
    }
    let vsix_content = resp.bytes().await?;

    // Save file
    fs::write(&file_path, &vsix_content)?;
    if verbose {
        println!("Saved in {file_path}");
    }

    Ok(())
}
