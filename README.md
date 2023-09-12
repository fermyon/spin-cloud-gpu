# Spin Cloud GPU

A plugin that allows you to develop your AI enabled app locally while using GPUs on Fermyon Cloud.

## Usage

```
spin cloud-gpu 0.1.0 (869ce65 2023-09-11)

USAGE:
    spin-cloud-gpu <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    connect    Connect to the fermyon-cloud-gpu Spin app to proxy your GPU
    destroy    Destroy the fermyon-cloud-gpu Spin app
    help       Print this message or the help of the given subcommand(s)
    init       Deploy the fermyon-cloud-gpu Spin app to act as a cloud GPU proxy
```

## Development

Build and install the plugin:

```sh
./create_plugin.sh && spin plugins install -f cloud-gpu.json -y
```
