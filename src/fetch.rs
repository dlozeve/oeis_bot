use crate::error::FetchError;
use crate::oeis::{Keyword, OeisEntry, OeisSequence};
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
    let entries: Vec<OeisEntry> = ureq::get("https://oeis.org/search")
        .query("q", format!("id:A{id:06}"))
        .query("fmt", "json")
        .call()?
        .body_mut()
        .read_json()?;
    let entry = entries.into_iter().next().ok_or(FetchError::NotFound(id))?;
    Ok(OeisSequence::from(entry))
}

/// Fetch a random sequence from the OEIS, excluding sequences with
/// one of the rejected keywords.
pub fn fetch_random() -> OeisSequence {
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
