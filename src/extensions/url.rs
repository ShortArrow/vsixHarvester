pub fn for_download(
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

pub const BASE_URL: &str = "https://marketplace.visualstudio.com/_apis/public/gallery/";

pub fn query_url() -> String {
    format!("{BASE_URL}/extensionquery")
}