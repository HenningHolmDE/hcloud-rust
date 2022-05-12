// Download OpenAPI spec for the Hetzner Cloud API

#[cfg(feature = "xshell")]
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

fn main() {
    #[cfg(feature = "xshell")]
    script::run().unwrap();

    #[cfg(not(feature = "xshell"))]
    panic!("This binary is only available when compiled with the `xshell` feature.");
}
