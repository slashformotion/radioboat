use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Station {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub volume: i64,
    #[serde(default)]
    pub muted: bool,
    pub stations: Vec<Station>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            volume: 80,
            muted: false,
            stations: Vec::new(),
        }
    }
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let expanded_path = shellexpand::tilde(path);
    let content = std::fs::read_to_string(expanded_path.as_ref())?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
