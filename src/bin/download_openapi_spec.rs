// Download OpenAPI spec for the Hetzner Cloud API

#[cfg(feature = "generator_scripts")]
mod script {
    use anyhow::Result;
    use hcloud::config::Config;
    use std::fs::{create_dir_all, OpenOptions};
    use std::io::Write;
    pub async fn run() -> Result<()> {
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

        create_dir_all(download_dir)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(hcloud_openapi_json)?;
        file.write_all(
            reqwest::get(hcloud_openapi_url)
                .await?
                .bytes()
                .await?
                .as_ref(),
        )?;

        Ok(())
    }
}

#[cfg_attr(feature = "generator_scripts", tokio::main)]
#[cfg(feature = "generator_scripts")]
async fn main() {
    script::run().await.unwrap();
}

#[cfg(not(feature = "generator_scripts"))]
fn main() {
    panic!("This binary is only available when compiled with the `generator_scripts` feature.");
}
