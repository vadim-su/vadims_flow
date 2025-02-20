#[derive(Debug, Clone)]
pub struct Info {
    pub version: String,
    pub rustc: String,
    pub build_date: String,
}

impl Default for Info {
    fn default() -> Self {
        let version = format!(
            "{}-{}",
            env!("CARGO_PKG_VERSION"),
            short_sha(env!("VERGEN_RUSTC_COMMIT_HASH")),
        );
        let rustc = env!("VERGEN_RUSTC_SEMVER").to_string();
        let build_date = env!("VERGEN_BUILD_DATE").to_string();

        return Self {
            version,
            rustc,
            build_date,
        };
    }
}

fn short_sha(sha: &str) -> String {
    sha.chars().take(7).collect()
}
