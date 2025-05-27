use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProofConfig {
    pub provingkey: String,
    pub verifyingkey: String,
    pub r1cs: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EthereumConfig {
    pub source_address: String,
    pub target_address: String,
    pub private_key: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub index: u64,
    #[serde(default)]
    pub registered: bool,
    pub bind_address: String,
    pub private_key: String,
    pub contract_address: String,
    pub ethereum: EthereumConfig,
    pub zkp: ProofConfig,
}

pub struct Node {
    pub config: Config,
}

impl Node {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn run(&self) {
        println!("Node running on {}", self.config.bind_address);
    }

    pub fn stop(&self) {
        println!("Node stopped");
    }
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    let config = serde_json::from_str(&data)?;
    Ok(config)
}
