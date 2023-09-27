# Spin Cloud GPU

The `spin cloud-gpu` plugin allows you to use GPUs on Fermyon Cloud while running your Spin app locally with `spin up`. 

Ever tested your AI-powered Spin app locally and spent quite a while waiting for your machine's compute to kick in...? Us too! That's why we built this plugin. Now with a few simple steps, you can use Fermyon Serverless AI GPUs to run inferencing and embedding requests for locally running Spin applications. Optionally, once local testing is complete and you're ready to run your application at scale, you can deploy to Fermyon Cloud with the standard `spin cloud deploy` command.  

### How it works

The `spin cloud-gpu` plugin, will be deploying a Spin application to Fermyon Cloud on your behalf that will serve as a proxy to access GPUs for your local Spin application. With the `spin cloud-gpu connect` command, you'll connect this proxy to your desired local application. You can always delete the proxy either via Fermyon Cloud UI or via `spin cloud-gpu destroy`. Read on to learn more about the prerequisites and the specific commands. 

## Prerequisites 

* You'll need a [Fermyon Cloud account](cloud.fermyon.com) if you don't have one already! Don't worry, this feature is available on our Developer Plan (free of charge). 
* Because you're accessing Fermyon Cloud's Serverless AI, you'll need to be enrolled in private beta. You can request access to the private beta with this [form](https://fibsu0jcu2g.typeform.com/to/mNzgXRvB).
* A local directory with a Spin application you'd like to connect to Fermyon Serverless AI. If you don't have one, you can create one via `spin new`

> Note that use of the `cloud-gpu` feature will count against your Fermyon Cloud quotas, specifically your [Spin app count](https://developer.fermyon.com/cloud/faq#quota-limits) quota as well as your [inferencing request](https://developer.fermyon.com/cloud/serverless-ai#quotas-and-service-limitations-for-fermyon-serverless-ai) quota. 


## Installation Steps

Build and install the plugin:

```sh
./create_plugin.sh && spin plugins install -f cloud-gpu.json -y
```

## Functionality

`spin cloud-gpu init` - deploy the fermyon-cloud-gpu Spin app to act as a cloud GPU proxy.
![](/img/spin-cloud-gpu-init.png)

`spin cloud-gpu connect` - connect to the fermyon-cloud-gpu Spin app to proxy your GPU. It is important you run this command from the local Spin application directory
![](/img/spin-cloud-gpu-connect.png)

`spin cloud-gpu destroy` - deletes the fermyon-cloud-gpu Spin application
![](/img/spin-cloud-gpu-destroy.png)

## Reference 

```
spin cloud-gpu 0.1.0 (869ce65 2023-09-11)

USAGE:
    spin cloud-gpu <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    connect    Connect to the fermyon-cloud-gpu Spin app to proxy your GPU
    destroy    Destroy the fermyon-cloud-gpu Spin app
    help       Print this message or the help of the given subcommand(s)
    init       Deploy the fermyon-cloud-gpu Spin app to act as a cloud GPU proxy
```