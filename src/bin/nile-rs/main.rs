mod cli;
mod commands;

use anyhow::{Ok, Result};
use dotenv::dotenv;

use clap::Parser;

use cli::Cli;
use commands::CliCommand;

#[tokio::main]
async fn main() -> Result<()> {
    // Load the environment variables from the ".env" file
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::RawCall(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::Compile(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::CounterfactualAddress(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::Declare(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::LegacyDeclare(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::Deploy(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::LegacyDeploy(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::Run(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::Send(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::Setup(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::Status(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::GetNonce(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::GetBalance(cmd) => {
            cmd.run().await?;
        }
        cli::Commands::GetAccounts(cmd) => {
            cmd.run().await?;
        }
    };

    Ok(())
}
