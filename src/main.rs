use clap::{Parser, Subcommand};
use std::process::Command as Cmd;
use uuid::Uuid;

/// TODO: Document
#[derive(Debug, Parser)]
#[clap(name = "spin-cloud-gpu", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    // TODO: Document all
    Init,
    Connect,
    Destroy,
}

fn main() -> Result<(), anyhow::Error> {
    // tracing_subscriber::fmt()
    //     .with_writer(std::io::stderr)
    //     .with_ansi(std::io::stderr().is_terminal())
    //     .init();

    let app = App::parse();
    match app.command {
        Command::Init => init(),
        Command::Connect => connect(),
        Command::Destroy => destroy(),
    }
}

fn init() -> Result<(), anyhow::Error> {
    // Generate a unique access key
    let _access_key = Uuid::new_v4().to_string();

    // std::env::set_current_dir("cloud-gpu")?; // TODO: Eww

    let spin_bin_path = std::env::var("SPIN_BIN_PATH")?; // TODO: Put in constant

    println!(
        "{:?}",
        &(std::env::current_dir()?.to_str().unwrap().to_owned() + "/cloud-gpu-app/spin.toml")
    );

    // Get the parent directory of the current executable path
    let spin_toml_path = std::env::current_exe()?
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/cloud-gpu-app/spin.toml";

    println!("deploying cloud-gpu application");
    let deploy_result = Cmd::new(spin_bin_path)
        .arg("deploy")
        .arg("-f")
        .arg(spin_toml_path)
        .output()?;
    println!("{}", String::from_utf8_lossy(&deploy_result.stdout));

    // Print instructions on how to put the access key in your environment
    Ok(())
}

fn connect() -> Result<(), anyhow::Error> {
    println!("connect");

    // TODO: Huh, idk...
    Ok(())
}

fn destroy() -> Result<(), anyhow::Error> {
    println!("destroy");
    // Destroy the cloud-gpu application
    Ok(())
}
