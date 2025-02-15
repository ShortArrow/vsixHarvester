use crate::extensions::info;
use crate::extensions::url;
use std::fs;
use std::path::Path;

fn get_target_platform(os_arch: Option<&str>, info: info::ExtensionInfo) -> Option<&str> {
    let current = std::env::consts::ARCH;
    let supporteds = info.architectures.clone();
    if supporteds.is_empty() {
        return None;
    }
    match os_arch {
        Some(arch) if supporteds.contains(&arch.to_string()) => Some(arch),
        Some(arch) => {
            eprintln!("Unsupported OS architecture: {arch} is not supported for {supporteds:?}");
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
        let current = std::env::consts::ARCH;
        let info = info::ExtensionInfo {
            architectures: vec![current.to_string(), "x64".to_string(), "x86".to_string()],
            ..Default::default()
        };
        assert_eq!(Some("x64"), get_target_platform(Some("x64"), info.clone()));
        assert_eq!(Some("x86"), get_target_platform(Some("x86"), info.clone()));
        assert_eq!(None, get_target_platform(Some("tekito"), info.clone()));
        assert_eq!(Some(current), get_target_platform(None, info));
    }
    #[test]
    fn test_when_current_is_not_supported() {
        let info = info::ExtensionInfo {
            architectures: vec!["x64".to_string(), "x86".to_string()],
            ..Default::default()
        };
        let current = std::env::consts::ARCH;
        assert_eq!(None, get_target_platform(None, info.clone()));
        assert_eq!(Some("x64"), get_target_platform(Some("x64"), info.clone()));
        assert_eq!(None, get_target_platform(Some(current), info));
    }
    #[test]
    fn test_when_no_supported_architectures() {
        let info = info::ExtensionInfo {
            architectures: vec![],
            ..Default::default()
        };
        assert_eq!(None, get_target_platform(None, info.clone()));
        assert_eq!(None, get_target_platform(Some("x64"), info));
    }
}

#[derive(Debug)]
struct ExtensionName {
    name: String,
    publisher: String,
}

fn validate_extension_name(name: &str) -> bool {
    let parts: Vec<&str> = name.split('.').collect();
    parts.len() == 2
}

fn parse_extension_name(name: &str) -> ExtensionName {
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

    let parsed_extension_name = parse_extension_name(extension);
    let publisher = &parsed_extension_name.publisher;
    let extension_name = &parsed_extension_name.name;

    // Get latest version
    let extension_info = info::get(publisher, extension_name, proxy, verbose).await?;
    let version = &extension_info.version.clone();
    if verbose {
        println!("Latest version of {extension}: {version:?}");
    }

    // Create download url
    let target_platform = get_target_platform(os_arch, extension_info);
    let download_url = url::for_download(publisher, extension_name, version, target_platform);
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
