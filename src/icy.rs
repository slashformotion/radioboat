#[derive(Debug, Clone, Default, PartialEq)]
pub struct IcyMetadata {
    pub name: Option<String>,
    pub genre: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub bitrate: Option<u32>,
    pub logo: Option<String>,
    pub country: Option<String>,
}

pub async fn fetch_icy_metadata(url: &str) -> Option<IcyMetadata> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .ok()?;

    let response = client
        .head(url)
        .header("Icy-MetaData", "1")
        .send()
        .await
        .ok()?;

    let headers = response.headers();

    let mut metadata = IcyMetadata::default();

    if let Some(name) = headers.get("icy-name").and_then(|v| v.to_str().ok()) {
        metadata.name = Some(name.to_string());
    }

    if let Some(genre) = headers.get("icy-genre").and_then(|v| v.to_str().ok()) {
        metadata.genre = Some(genre.to_string());
    }

    if let Some(desc) = headers.get("icy-description").and_then(|v| v.to_str().ok()) {
        metadata.description = Some(desc.to_string());
    }

    if let Some(url) = headers.get("icy-url").and_then(|v| v.to_str().ok()) {
        metadata.url = Some(url.to_string());
    }

    if let Some(logo) = headers.get("icy-logo").and_then(|v| v.to_str().ok()) {
        metadata.logo = Some(logo.to_string());
    }

    if let Some(country) = headers.get("icy-country-code").and_then(|v| v.to_str().ok()) {
        metadata.country = Some(country.to_string());
    }

    if let Some(br) = headers.get("icy-br").and_then(|v| v.to_str().ok()) {
        metadata.bitrate = br.parse().ok();
    }

    let has_any = metadata.name.is_some()
        || metadata.genre.is_some()
        || metadata.description.is_some()
        || metadata.url.is_some();

    if has_any {
        Some(metadata)
    } else {
        None
    }
}
