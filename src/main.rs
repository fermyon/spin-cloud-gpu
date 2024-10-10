use anyhow::{anyhow, Result};
use clap::Parser;
use dialoguer::Confirm;
use regex::Regex;
use std::process::Command as Cmd;
use uuid::Uuid;

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
    Init,
    /// Rotate the Auth Token for your existing Fermyon Cloud GPU
    RotateToken,
    /// Destroy the Fermyon Cloud GPU Spin App.
    Destroy,
}

fn main() -> Result<(), anyhow::Error> {
    match App::parse() {
        App::Init => init(),
        App::RotateAuthToken => rotate_auth_token(),
        // App::Connect => connect(),
        App::Destroy => destroy(),
    }
}

fn init() -> Result<(), anyhow::Error> {
    println!("Deploying Fermyon Cloud GPU Spin App ...");

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

    let url = match extract_url(&String::from_utf8_lossy(&result.stdout)) {
        Ok(val) => val,
        Err(_) => "<Insert url from cloud dashboard>".to_owned(),
    };

    print_runtime_config(url, auth_token);

    Ok(())
}

fn rotate_auth_token() -> Result<(), anyhow::Error> {
    let confirmation = Confirm::new()
        .with_prompt("Do you really want to rotate the Auth Token for Fermyon Cloud GPU? (Existing Spin Apps using your instance of Fermyon Cloud GPU must be updated)")
        .interact()
        .unwrap();

    if !confirmation {
        println!("Operation canceled! Auth Token for Fermyon Cloud GPU has not been rotated.");
        return Ok(());
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

    let url = match extract_url(&String::from_utf8_lossy(&result.stdout)) {
        Ok(val) => val,
        Err(_) => "<Insert url from cloud dashboard>".to_owned(),
    };

    println!("\nAuth Token for Fermyon Cloud GPU rotated!\n");
    print_runtime_config(url, auth_token);
    Ok(())
}

// fn connect() -> Result<(), anyhow::Error> {
//     println!("Connecting to fermyon-cloud-gpu Spin app ...");

//     let auth_token = generate_auth_token();

//     let result = Cmd::new(spin_bin_path()?)
//         .arg("cloud")
//         .arg("variables")
//         .arg("set")
//         .arg(format!("auth_token={auth_token}"))
//         .arg("--app")
//         .arg("fermyon-cloud-gpu")
//         .output()?;

//     if !result.status.success() {
//         return Err(anyhow!(
//             "Failed to update auth_token in fermyon-cloud-gpu: {}",
//             String::from_utf8_lossy(&result.stderr)
//         ));
//     }

//     print_runtime_config(auth_token);

//     Ok(())
// }

fn destroy() -> Result<(), anyhow::Error> {
    println!("Destroying Fermyon Cloud GPU Spin App ...");

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

fn print_runtime_config(url: String, auth_token: String) {
    println!("Add the following configuration to your runtime configuration file.");
    println!(
        r#"
[llm_compute]
type = "remote_http"
url = "{url}"
auth_token = "{auth_token}"
"#
    );
    println!("\nOnce added, you can spin up with the following argument --runtime-config-file <path/to/runtime/config>.");
}

fn extract_url(input: &str) -> Result<String> {
    let re = Regex::new(r"fermyon-cloud-gpu: (https://[^\s]+)")?;
    if let Some(captures) = re.captures(input) {
        Ok(captures[1].to_string())
    } else {
        Err(anyhow!("Failed to extra url"))
    }
}
