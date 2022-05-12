#[cfg(feature = "xshell")]
pub struct Config {
    pub download_dir: &'static str,

    pub generator_version: &'static str,
    pub generator_url: String,
    pub generator_jar: String,

    pub hcloud_openapi_version: &'static str,
    pub hcloud_openapi_url: String,
    pub hcloud_openapi_json: String,
}
impl Config {
    pub fn default() -> Self {
        const DOWNLOAD_DIR: &str = "generator_files";
        const GENERATOR_VERSION: &str = "5.4.0";
        const HCLOUD_OPENAPI_VERSION: &str = "0.7.0";

        Config {
            download_dir: DOWNLOAD_DIR,
            generator_version: GENERATOR_VERSION,
            generator_url: format!("https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{}/openapi-generator-cli-{}.jar", GENERATOR_VERSION,GENERATOR_VERSION),
            generator_jar: format!("{}/openapi-generator-cli-{}.jar", DOWNLOAD_DIR, GENERATOR_VERSION),
            hcloud_openapi_version: HCLOUD_OPENAPI_VERSION,
            hcloud_openapi_url: format!(
            "https://github.com/MaximilianKoestler/hcloud-openapi/releases/download/v{}/hcloud.json",HCLOUD_OPENAPI_VERSION
            ),
            hcloud_openapi_json: format!("{}/hcloud_{}.json", DOWNLOAD_DIR, HCLOUD_OPENAPI_VERSION),
        }
    }
}
