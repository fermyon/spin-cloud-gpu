# Fermyon Cloud GPU

Proxies inferencing and embedding requests to the Fermyon Cloud Serverless AI feature for faster local development. This app is meant to be deployed by the `spin cloud-gpu` plugin.

## Setup and Testing

```sh
npm install
spin build
# If testing locally copy the ai_models into .spin/ai_models
```

To test it locally:

```sh
SPIN_CONFIG_AUTH_TOKEN=your-auth-token spin up
curl -H 'Authorization: bearer your-auth-token' http://localhost:3000/infer --data '{"model": "llama2-chat", "prompt": "My prompt"}'
```

To test it in the cloud:

```sh
spin deploy --variable auth_token=your-auth-token
curl -H 'Authorization: bearer your-auth-token' https://appdomain.fermyon.app/infer --data '{"model": "llama2-chat", "prompt": "My prompt"}'
```
