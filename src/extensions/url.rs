pub fn for_download(
    publisher: &str,
    extension_name: &str,
    version: &str,
    target_platform: Option<String>,
) -> String {
    match target_platform {
        Some(platform) => format!(
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension_name}/{version}/vspackage?targetPlatform={platform}",
        ),
        None => format!(
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension_name}/{version}/vspackage",
        ),
    }
}

pub const BASE_URL: &str = "https://marketplace.visualstudio.com/_apis/public/gallery/";

pub fn query_url() -> String {
    format!("{BASE_URL}/extensionquery")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_download_no_target_platform() {
        let publisher = "another_publisher";
        let extension_name = "another_extension";
        let version = "1.0.0";
        let target_platform = None;
        let expected = format!(
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension_name}/{version}/vspackage",
        );
        assert_eq!(
            for_download(publisher, extension_name, version, target_platform),
            expected
        );
    }

    #[test]
    fn test_for_download_with_target_platform() {
        let publisher = "another_publisher";
        let extension_name = "another_extension";
        let version = "1.0.0";
        let target_platform = Some("win32-x64".to_string());
        let unwraped_target_platform = target_platform.clone().unwrap();
        let expected = format!(
            "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{extension_name}/{version}/vspackage?targetPlatform={unwraped_target_platform}",
        );
        assert_eq!(
            for_download(publisher, extension_name, version, target_platform),
            expected
        );
    }

    #[test]
    fn test_query_url() {
        let expected = format!("{BASE_URL}/extensionquery");
        assert_eq!(query_url(), expected);
    }
}