use crate::oeis::OeisSequence;
use ureq::Error;

/// Format a sequence as a status message.
pub fn format_status(seq: &OeisSequence) -> String {
    let data: Vec<String> = seq.data.iter().map(|n| n.to_string()).collect();
    format!(
        "OEIS sequence A{:06}\n{}\n\n{}\n\nhttps://oeis.org/A{}",
        seq.number,
        seq.name,
        data.join(", "),
        seq.number,
    )
}

/// Post a status to a Mastodon instance.
///
/// `instance_url` is the base URL (e.g. `https://mastodon.social`).
/// `token` is a Bearer access token with `write:statuses` scope.
pub fn post_status(instance_url: &str, token: &str, status: &str) -> Result<(), Error> {
    let url = format!("{}/api/v1/statuses", instance_url.trim_end_matches('/'));
    ureq::post(&url)
        .header("Authorization", &format!("Bearer {token}"))
        .send_form([("status", status)])?;
    Ok(())
}
