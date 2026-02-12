mod error;
mod oeis;

use error::FetchError;
use oeis::{Keyword, OeisEntry, OeisSequence};
use rand::Rng;

const MAX_SEQUENCE_ID: u64 = 380_000;

const REJECTED_KEYWORDS: &[Keyword] = &[
    Keyword::Dead,
    Keyword::Dumb,
    Keyword::Dupe,
    Keyword::Less,
    Keyword::Obsc,
    Keyword::Probation,
    Keyword::Uned,
];

/// Fetch a sequence from oeis.org by its A-number (e.g. `fetch(250000)`
/// retrieves A250000).
pub fn fetch(id: u64) -> Result<OeisSequence, FetchError> {
    let url = format!("https://oeis.org/search?q=id:A{id:06}&fmt=json");
    let body = ureq::get(&url).call()?.body_mut().read_to_string()?;
    let entries: Vec<OeisEntry> = serde_json::from_str(&body)?;
    let entry = entries.into_iter().next().ok_or(FetchError::NotFound(id))?;
    Ok(OeisSequence::from(entry))
}

fn fetch_random() -> OeisSequence {
    let mut rng = rand::rng();
    loop {
        let id = rng.random_range(1..=MAX_SEQUENCE_ID);
        let seq = match fetch(id) {
            Ok(seq) => seq,
            Err(FetchError::NotFound(_)) => continue,
            Err(e) => panic!("{e}"),
        };
        if seq.keyword.iter().any(|kw| REJECTED_KEYWORDS.contains(kw)) {
            continue;
        }
        return seq;
    }
}

fn main() {
    let seq = fetch_random();
    println!("A{:06}: {}", seq.number, seq.name);
    println!("First terms: {:?}", &seq.data[..15.min(seq.data.len())]);
    println!("Keywords: {:?}", seq.keyword);
}
