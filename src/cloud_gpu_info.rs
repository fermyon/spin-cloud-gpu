use regex::Regex;
use serde::Serialize;

const FALLBACK_URL: &str = "<Insert url from Fermyon Cloud dashboard>";

#[derive(Serialize)]
pub(crate) struct CloudGpuInfo {
    llm_type: String,
    pub url: String,
    pub auth_token: String,
}

fn extract_url(input: &str) -> String {
    let Ok(re) = Regex::new(r"fermyon-cloud-gpu: (https://[^\s]+)") else {
        return String::from(FALLBACK_URL);
    };
    let Some(captures) = re.captures(input) else {
        return String::from(FALLBACK_URL);
    };
    captures[1].to_string()
}

impl CloudGpuInfo {
    pub(crate) fn new(auth_token: String, url: &str) -> Self {
        Self {
            llm_type: String::from("remote_http"),
            auth_token,
            url: extract_url(url),
        }
    }

    pub(crate) fn print(&self, json: bool, toml: bool) {
        println!(
            "{}",
            match (json, toml) {
                (true, false) => self.to_json(),
                (false, true) => self.to_toml(),
                (_, _) => self.to_string(),
            }
        )
    }

    fn to_string(&self) -> String {
        format!(
            r#"Add the following configuration to your runtime configuration file.

[llm_compute]
type = "{}"
url = "{}"
auth_token = "{}"

Once added, you can spin up with the following argument --runtime-config-file <path/to/runtime/config>.
"#,
            self.llm_type, self.url, self.auth_token,
        )
    }

    fn to_toml(&self) -> String {
        let container = self.wrap_with_container();
        toml::to_string(&container).unwrap()
    }

    fn wrap_with_container(&self) -> ConfigContainer {
        ConfigContainer { llm_compute: self }
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Serialize)]
struct ConfigContainer<'a> {
    pub llm_compute: &'a CloudGpuInfo,
}
