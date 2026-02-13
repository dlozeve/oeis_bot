mod error;
mod fetch;
mod mastodon;
mod oeis;

use std::env;

fn main() {
    let seq = fetch::fetch_random();
    let status = mastodon::format_status(&seq);

    let instance_url = env::var("MASTODON_INSTANCE_URL")
        .expect("MASTODON_INSTANCE_URL environment variable must be set");
    let token = env::var("MASTODON_ACCESS_TOKEN")
        .expect("MASTODON_ACCESS_TOKEN environment variable must be set");

    mastodon::post_status(&instance_url, &token, &status)
        .expect("failed to post status to Mastodon");
}
