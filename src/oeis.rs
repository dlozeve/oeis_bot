use serde::Deserialize;

fn join_lines(v: Vec<String>) -> String {
    v.join("\n")
}

fn parse_data(s: &str) -> Vec<i64> {
    s.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("invalid integer in OEIS data field"))
        .collect()
}

/// An OEIS sequence in a form convenient for manipulation.
#[derive(Debug, Clone)]
pub struct OeisSequence {
    /// The A-number (e.g. 250000 for A250000).
    pub number: u64,
    /// Old-style handbook ID (e.g. "M0692 N0256").
    pub id: Option<String>,
    /// The sequence values.
    pub data: Vec<i64>,
    /// Human-readable name/description.
    pub name: String,
    /// Commentary and observations.
    pub comment: String,
    /// Bibliographic references.
    pub reference: String,
    /// URLs and resource links.
    pub link: String,
    /// Mathematical formulas and bounds.
    pub formula: String,
    /// Worked examples.
    pub example: String,
    /// Maple code.
    pub maple: String,
    /// Mathematica code.
    pub mathematica: String,
    /// Programs in other languages (PARI, Python, etc.).
    pub program: String,
    /// Cross-references to related sequences.
    pub xref: String,
    /// Comma-separated keyword tags (e.g. "nonn,core,nice,hard").
    pub keyword: String,
    /// Offset information (e.g. "0,5").
    pub offset: String,
    /// Author attribution.
    pub author: String,
    /// Extension and edit history.
    pub ext: String,
    /// Number of references.
    pub references: u64,
    /// Revision number.
    pub revision: u64,
    /// Last modification timestamp (ISO 8601).
    pub time: String,
    /// Creation timestamp (ISO 8601).
    pub created: String,
}

impl From<OeisEntry> for OeisSequence {
    fn from(e: OeisEntry) -> Self {
        Self {
            number: e.number,
            id: e.id,
            data: parse_data(&e.data),
            name: e.name,
            comment: join_lines(e.comment),
            reference: join_lines(e.reference),
            link: join_lines(e.link),
            formula: join_lines(e.formula),
            example: join_lines(e.example),
            maple: join_lines(e.maple),
            mathematica: join_lines(e.mathematica),
            program: join_lines(e.program),
            xref: join_lines(e.xref),
            keyword: e.keyword,
            offset: e.offset,
            author: e.author,
            ext: join_lines(e.ext),
            references: e.references,
            revision: e.revision,
            time: e.time,
            created: e.created,
        }
    }
}

/// Raw JSON representation of a single OEIS sequence entry.
///
/// The API response is a `Vec<OeisEntry>`. Convert to [`OeisSequence`] for
/// easier manipulation.
#[derive(Debug, Clone, Deserialize)]
pub struct OeisEntry {
    /// The A-number (e.g. 250000 for A250000).
    pub number: u64,

    /// Old-style handbook ID (e.g. "M0692 N0256"). Only present for older sequences.
    #[serde(default)]
    pub id: Option<String>,

    /// Comma-separated sequence values.
    pub data: String,

    /// Human-readable name/description of the sequence.
    pub name: String,

    /// Commentary and observations about the sequence.
    #[serde(default)]
    pub comment: Vec<String>,

    /// Bibliographic references (books, papers).
    #[serde(default)]
    pub reference: Vec<String>,

    /// URLs and resource links (may contain HTML).
    #[serde(default)]
    pub link: Vec<String>,

    /// Mathematical formulas and bounds.
    #[serde(default)]
    pub formula: Vec<String>,

    /// Worked examples illustrating the sequence.
    #[serde(default)]
    pub example: Vec<String>,

    /// Maple code.
    #[serde(default)]
    pub maple: Vec<String>,

    /// Mathematica code.
    #[serde(default)]
    pub mathematica: Vec<String>,

    /// Programs in other languages (PARI, Python, etc.).
    #[serde(default)]
    pub program: Vec<String>,

    /// Cross-references to related sequences.
    #[serde(default)]
    pub xref: Vec<String>,

    /// Comma-separated keyword tags (e.g. "nonn,core,nice,hard").
    pub keyword: String,

    /// Offset information (e.g. "0,5" meaning a(0) is the first term, a(5) is
    /// the first term > 1).
    pub offset: String,

    /// Author attribution.
    pub author: String,

    /// Extension and edit history entries.
    #[serde(default)]
    pub ext: Vec<String>,

    /// Number of references.
    pub references: u64,

    /// Revision number.
    pub revision: u64,

    /// Last modification timestamp (ISO 8601).
    pub time: String,

    /// Creation timestamp (ISO 8601).
    pub created: String,
}
