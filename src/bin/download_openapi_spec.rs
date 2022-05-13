// Download OpenAPI spec for the Hetzner Cloud API

#[cfg(feature = "generator_scripts")]
mod script {
    use anyhow::Result;
    use hcloud::config::Config;
    use xshell::{cmd, Shell};
    pub fn run() -> Result<()> {
        let Config {
            download_dir,
            hcloud_openapi_version,
            hcloud_openapi_url,
            hcloud_openapi_json,
            ..
        } = Config::default();
        println!(
            "Downloading version {} of the OpenAPI spec for the Hetzner Cloud API...",
            hcloud_openapi_version
        );

        let sh = Shell::new()?;
        cmd!(sh, "mkdir -p {download_dir}").run()?;

        cmd!(sh, "curl -L {hcloud_openapi_url} -o {hcloud_openapi_json}").run()?;

        Ok(())
    }
}

#[cfg_attr(feature = "generator_scripts", tokio::main)]
#[cfg(feature = "generator_scripts")]
async fn main() {
    script::run().unwrap();
}

#[cfg(not(feature = "generator_scripts"))]
fn main() {
    panic!("This binary is only available when compiled with the `generator_scripts` feature.");
}
