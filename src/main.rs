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
    let auth_token = Uuid::new_v4().to_string();

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
        .arg("--variable")
        .arg(format!("auth_token={auth_token}"))
        .output()?;
    println!("{}", String::from_utf8_lossy(&deploy_result.stdout));

    println!("export asdf={auth_token}");

    // Print instructions on how to put the access key in your environment
    Ok(())
}

fn connect() -> Result<(), anyhow::Error> {
    let auth_token = Uuid::new_v4().to_string();

    let spin_bin_path = std::env::var("SPIN_BIN_PATH")?; // TODO: Put in constant

    // Get the parent directory of the current executable path
    let spin_toml_path = std::env::current_exe()?
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/cloud-gpu-app/spin.toml";

    println!("updating fermyon-cloud-gpu config");
    let deploy_result = Cmd::new(spin_bin_path)
        .arg("cloud")
        .arg("variables")
        .arg("set")
        .arg(format!("auth_token={auth_token}"))
        .arg("--app")
        .arg("fermyon-cloud-gpu")
        .output()?;
    println!("{}", String::from_utf8_lossy(&deploy_result.stdout));

    println!("export asdf={auth_token}");

    // Print instructions on how to put the access key in your environment
    Ok(())
}

fn destroy() -> Result<(), anyhow::Error> {
    let spin_bin_path = std::env::var("SPIN_BIN_PATH")?; // TODO: Put in constant

    println!("updating fermyon-cloud-gpu config");
    let deploy_result = Cmd::new(spin_bin_path)
        .arg("cloud")
        .arg("apps")
        .arg("delete")
        .arg("fermyon-cloud-gpu")
        .output()?;
    println!("{}", String::from_utf8_lossy(&deploy_result.stdout));

    // Print instructions on how to put the access key in your environment
    Ok(())
}

// TODO: Actually confirm running command didn't blow up
