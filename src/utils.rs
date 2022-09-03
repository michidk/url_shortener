use rand::seq::SliceRandom;
use serde_json::{json, Map};
use serde_json::value::Value;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

const GITHUB_URL: &str = "https://github.com/michidk/url_shortener";

pub fn get_rnd_spell() -> String {
    let slugs = vec![
        "As simple as it gets",
        "Dead simple",
        "Simple setup, short links",
        "✂️✂️✂️",
    ];
    let selection = slugs.choose(&mut rand::thread_rng());
    String::from(*selection.unwrap())
}

pub fn get_app_info(
) -> Map<String, Value> {
    json!({
        "name": "URL Shortener",
        "version": built_info::PKG_VERSION,
        "github_url": GITHUB_URL,
        "git_hash": built_info::GIT_COMMIT_HASH,
    })
    .as_object()
    .expect("Error building app_info struct")
    .to_owned()
}
