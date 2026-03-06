use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Station {
    pub name: String,
    pub url: String,
    #[serde(skip)]
    pub is_remote: bool,
    #[serde(skip)]
    pub source: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Import {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub volume: i64,
    #[serde(default)]
    pub muted: bool,
    #[serde(default)]
    pub imports: Vec<Import>,
    pub stations: Vec<Station>,
}

#[derive(Debug, Deserialize)]
pub struct RemoteStationList {
    pub stations: Vec<Station>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: 80,
            muted: false,
            imports: Vec::new(),
            stations: Vec::new(),
        }
    }
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let expanded_path = shellexpand::tilde(path);
    let content = std::fs::read_to_string(expanded_path.as_ref())?;
    let mut config: Config = toml::from_str(&content)?;
    for station in &mut config.stations {
        station.is_remote = false;
        station.source = "local".to_string();
    }
    Ok(config)
}

pub async fn fetch_remote_stations(imports: &[Import]) -> (Vec<Station>, Vec<String>) {
    let mut all_stations: Vec<Station> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for import in imports {
        match fetch_single_source(&import.url).await {
            Ok(mut stations) => {
                for station in &mut stations {
                    station.is_remote = true;
                    station.source.clone_from(&import.name);
                }
                all_stations.extend(stations);
            }
            Err(e) => {
                errors.push(format!("Failed to import '{}': {}", import.name, e));
            }
        }
    }

    (all_stations, errors)
}

async fn fetch_single_source(source: &str) -> anyhow::Result<Vec<Station>> {
    let expanded = shellexpand::tilde(source);
    let expanded_str = expanded.as_ref();

    let content = if expanded_str.starts_with("http://") || expanded_str.starts_with("https://") {
        fetch_http(expanded_str).await?
    } else {
        std::fs::read_to_string(expanded_str)?
    };

    let remote_list: RemoteStationList = toml::from_str(&content)?;
    Ok(remote_list.stations)
}

async fn fetch_http(url: &str) -> anyhow::Result<String> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

pub fn merge_stations(local: Vec<Station>, remote: Vec<Station>) -> Vec<Station> {
    let mut merged: Vec<Station> = local;

    let local_urls: std::collections::HashSet<String> =
        merged.iter().map(|s| s.url.clone()).collect();

    let mut remote_by_url: std::collections::HashMap<String, Station> =
        std::collections::HashMap::new();
    for station in remote {
        if !local_urls.contains(&station.url) {
            remote_by_url.insert(station.url.clone(), station);
        }
    }

    merged.extend(remote_by_url.into_values());

    merged.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    merged
}
