use anyhow::{anyhow, Result};
use clap::Parser;
use cloud_gpu_info::CloudGpuInfo;
use dialoguer::Confirm;
use std::{process::Command as Cmd, str::FromStr};
use uuid::Uuid;

mod cloud_gpu_info;

/// Returns build information, similar to: 0.1.0 (2be4034 2022-03-31).
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_GIT_COMMIT_DATE"),
    ")"
);

#[derive(Debug, Parser)]
#[clap(name = "spin cloud-gpu", version = VERSION)]
pub enum App {
    /// Deploy the Fermyon Cloud GPU Spin App to act as a cloud GPU proxy.
    Init(InitOptions),
    /// Rotate the Auth Token for your existing Fermyon Cloud GPU
    RotateToken(RotateOptions),
    /// Destroy the Fermyon Cloud GPU Spin App.
    Destroy,
}

#[derive(Debug, Parser)]
pub struct RotateOptions {
    /// Automatically confirm token rotation
    #[clap(long = "yes", short = 'y', takes_value = false)]
    pub yes: bool,
    /// Print formatted output. Supported formats are json and toml
    #[clap(long = "format")]
    pub format: Option<OutputFormat>,
}

#[derive(Debug, Parser)]
pub enum OutputFormat {
    Json,
    Toml,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "toml" => Ok(OutputFormat::Toml),
            _ => anyhow::bail!("Invalid format provided. Valid formats are toml and json"),
        }
    }
}

#[derive(Parser, Debug)]
pub struct InitOptions {
    /// Print formatted output. Supported formats are json and toml
    #[clap(long = "format")]
    pub format: Option<OutputFormat>,
}

fn main() -> Result<(), anyhow::Error> {
    match App::parse() {
        App::Init(options) => init(options),
        App::RotateToken(options) => rotate_auth_token(options),
        // App::Connect => connect(),
        App::Destroy => destroy(),
    }
}

fn init(options: InitOptions) -> Result<(), anyhow::Error> {
    eprintln!("Deploying Fermyon Cloud GPU Spin App ...");

    let auth_token = generate_auth_token();

    let result = Cmd::new(spin_bin_path()?)
        .arg("deploy")
        .arg("-f")
        .arg(spin_toml_path()?)
        .arg("--variable")
        .arg(format!("auth_token={auth_token}"))
        .output()?;

    if !result.status.success() {
        return Err(anyhow!(
            "Failed to deploy Fermyon Cloud GPU: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    let url = &String::from_utf8_lossy(&result.stdout);

    let info = CloudGpuInfo::new(auth_token, url);
    info.print(options.format);
    Ok(())
}

fn rotate_auth_token(options: RotateOptions) -> Result<(), anyhow::Error> {
    if !options.yes {
        let confirmation = Confirm::new()
            .default(false)
        .with_prompt("Do you really want to rotate the Auth Token for Fermyon Cloud GPU? (Existing Spin Apps using your instance of Fermyon Cloud GPU must be updated)")
            .interact()
            .unwrap();

        if !confirmation {
            eprintln!("Operation canceled! Auth Token for Fermyon Cloud GPU has not been rotated.");
            return Ok(());
        }
    }

    let auth_token = generate_auth_token();
    let result = Cmd::new(spin_bin_path()?)
        .arg("cloud")
        .arg("variables")
        .arg("set")
        .arg("--app")
        .arg("fermyon-cloud-gpu")
        .arg(format!("auth_token={auth_token}"))
        .output()?;

    if !result.status.success() {
        return Err(anyhow!(
            "Failed to update Auth Token for Fermyon Cloud GPU: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    let url = &String::from_utf8_lossy(&result.stdout);

    let info = CloudGpuInfo::new(auth_token, url);
    eprintln!("\nAuth Token for Fermyon Cloud GPU rotated!\n");
    info.print(options.format);
    Ok(())
}

fn destroy() -> Result<(), anyhow::Error> {
    eprintln!("Destroying Fermyon Cloud GPU Spin App ...");

    let result = Cmd::new(spin_bin_path()?)
        .arg("cloud")
        .arg("apps")
        .arg("delete")
        .arg("fermyon-cloud-gpu")
        .output()?;

    if !result.status.success() {
        return Err(anyhow!(
            "Failed to delete Fermyon Cloud GPU: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(())
}

fn generate_auth_token() -> String {
    Uuid::new_v4().to_string()
}

fn spin_bin_path() -> Result<String> {
    Ok(std::env::var("SPIN_BIN_PATH")?)
}

/// Returns the path to the spin.toml file of the fermyon-cloud-gpu Spin app.
fn spin_toml_path() -> Result<String> {
    Ok(std::env::current_exe()?
        .parent()
        .unwrap()
        .to_str()
        .ok_or(anyhow!("Could not get parent dir of executable"))?
        .to_owned()
        + "/fermyon-cloud-gpu/spin.toml")
}
