use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

const READ_BUFFER_SIZE: usize = 64 * 1024;

#[derive(Debug, Parser)]
#[command(
    name = "reliquary",
    version,
    about = "Hardware-backed evidence tooling for RELIQUARY"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Calculate the SHA-256 digest of an artifact
    Hash {
        /// Path to the artifact
        artifact: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hash { artifact } => {
            let digest = calculate_sha256(&artifact)?;

            println!("Artifact: {}", artifact.display());
            println!("SHA256:   {digest}");
        }
    }

    Ok(())
}

fn calculate_sha256(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; READ_BUFFER_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let digest = hasher.finalize();

    Ok(hex::encode(digest))
}
