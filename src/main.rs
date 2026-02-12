mod error;
mod oeis;

use error::FetchError;
use oeis::{OeisEntry, OeisSequence};

/// Fetch a sequence from oeis.org by its A-number (e.g. `fetch(250000)`
/// retrieves A250000).
pub fn fetch(id: u64) -> Result<OeisSequence, FetchError> {
    let url = format!("https://oeis.org/search?q=id:A{id:06}&fmt=json");
    let body = ureq::get(&url).call()?.body_mut().read_to_string()?;
    let entries: Vec<OeisEntry> = serde_json::from_str(&body)?;
    let entry = entries.into_iter().next().ok_or(FetchError::NotFound(id))?;
    Ok(OeisSequence::from(entry))
}

fn main() {
    let seq = fetch(250000).unwrap();
    println!("A{:06}: {}", seq.number, seq.name);
    println!("First terms: {:?}", &seq.data[..15.min(seq.data.len())]);
    println!("Keywords: {:?}", seq.keyword);
}
