use clap::Parser;
use rust_node::{load_config, Node};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'c', long = "config", default_value = "./configs/config.example.json")]
    config: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = load_config(&args.config)?;
    let node = Node::new(config);
    node.run();
    // In a real implementation we would block on signal handling
    Ok(())
}
