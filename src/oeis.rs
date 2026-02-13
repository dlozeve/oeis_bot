use num_bigint::BigInt;
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

/// An OEIS keyword tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    /// Sequence is dependent on base used.
    Base,
    /// Sequence is too short to do any analysis with.
    Bref,
    /// Recently modified (assigned automatically).
    Changed,
    /// A continued fraction expansion of a number.
    Cofr,
    /// A decimal expansion of a number.
    Cons,
    /// An important sequence.
    Core,
    /// An erroneous or duplicated sequence.
    Dead,
    /// An unimportant sequence.
    Dumb,
    /// Duplicate of another sequence.
    Dupe,
    /// It is easy to produce terms of this sequence.
    Easy,
    /// A fixed sequence for some transformation.
    Eigen,
    /// A finite sequence.
    Fini,
    /// Numerators or denominators of a sequence of rationals.
    Frac,
    /// All terms of the sequence are given (implies `Fini`).
    Full,
    /// Next term is not known and may be hard to find.
    Hard,
    /// A sequence worth listening to.
    Hear,
    /// Less interesting; unlikely to be the intended match.
    Less,
    /// A sequence with an interesting graph.
    Look,
    /// More terms are needed.
    More,
    /// Multiplicative: a(mn) = a(m)*a(n) if gcd(m,n) = 1.
    Mult,
    /// Recently added (assigned automatically).
    New,
    /// An exceptionally nice sequence.
    Nice,
    /// All terms are nonnegative.
    Nonn,
    /// Obscure, better description needed.
    Obsc,
    /// Included on probation; may be deleted later.
    Probation,
    /// Sequence contains negative numbers.
    Sign,
    /// An irregular (or funny-shaped) triangle of numbers read by rows.
    Tabf,
    /// A regular triangle or square array read by rows or antidiagonals.
    Tabl,
    /// Not yet edited; requires editorial review.
    Uned,
    /// Little is known; an unsolved problem.
    Unkn,
    /// Counts walks or self-avoiding paths.
    Walk,
    /// Depends on words for the sequence in some language.
    Word,
}

impl Keyword {
    pub fn as_str(self) -> &'static str {
        match self {
            Keyword::Base => "base",
            Keyword::Bref => "bref",
            Keyword::Changed => "changed",
            Keyword::Cofr => "cofr",
            Keyword::Cons => "cons",
            Keyword::Core => "core",
            Keyword::Dead => "dead",
            Keyword::Dumb => "dumb",
            Keyword::Dupe => "dupe",
            Keyword::Easy => "easy",
            Keyword::Eigen => "eigen",
            Keyword::Fini => "fini",
            Keyword::Frac => "frac",
            Keyword::Full => "full",
            Keyword::Hard => "hard",
            Keyword::Hear => "hear",
            Keyword::Less => "less",
            Keyword::Look => "look",
            Keyword::More => "more",
            Keyword::Mult => "mult",
            Keyword::New => "new",
            Keyword::Nice => "nice",
            Keyword::Nonn => "nonn",
            Keyword::Obsc => "obsc",
            Keyword::Probation => "probation",
            Keyword::Sign => "sign",
            Keyword::Tabf => "tabf",
            Keyword::Tabl => "tabl",
            Keyword::Uned => "uned",
            Keyword::Unkn => "unkn",
            Keyword::Walk => "walk",
            Keyword::Word => "word",
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct ParseKeywordError(pub String);

impl fmt::Display for ParseKeywordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown OEIS keyword: {:?}", self.0)
    }
}

impl std::error::Error for ParseKeywordError {}

impl FromStr for Keyword {
    type Err = ParseKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "base" => Ok(Keyword::Base),
            "bref" => Ok(Keyword::Bref),
            "changed" => Ok(Keyword::Changed),
            "cofr" => Ok(Keyword::Cofr),
            "cons" => Ok(Keyword::Cons),
            "core" => Ok(Keyword::Core),
            "dead" => Ok(Keyword::Dead),
            "dumb" => Ok(Keyword::Dumb),
            "dupe" => Ok(Keyword::Dupe),
            "easy" => Ok(Keyword::Easy),
            "eigen" => Ok(Keyword::Eigen),
            "fini" => Ok(Keyword::Fini),
            "frac" => Ok(Keyword::Frac),
            "full" => Ok(Keyword::Full),
            "hard" => Ok(Keyword::Hard),
            "hear" => Ok(Keyword::Hear),
            "less" => Ok(Keyword::Less),
            "look" => Ok(Keyword::Look),
            "more" => Ok(Keyword::More),
            "mult" => Ok(Keyword::Mult),
            "new" => Ok(Keyword::New),
            "nice" => Ok(Keyword::Nice),
            "nonn" => Ok(Keyword::Nonn),
            "obsc" => Ok(Keyword::Obsc),
            "probation" => Ok(Keyword::Probation),
            "sign" => Ok(Keyword::Sign),
            "tabf" => Ok(Keyword::Tabf),
            "tabl" => Ok(Keyword::Tabl),
            "uned" => Ok(Keyword::Uned),
            "unkn" => Ok(Keyword::Unkn),
            "walk" => Ok(Keyword::Walk),
            "word" => Ok(Keyword::Word),
            other => Err(ParseKeywordError(other.to_owned())),
        }
    }
}

fn join_lines(v: Vec<String>) -> String {
    v.join("\n")
}

fn parse_data(s: &str) -> Vec<BigInt> {
    s.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("invalid integer in OEIS data field"))
        .collect()
}

fn parse_keywords(s: &str) -> Vec<Keyword> {
    s.split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("unknown OEIS keyword"))
        .collect()
}

/// An OEIS sequence in a form convenient for manipulation.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OeisSequence {
    /// The A-number (e.g. 250000 for A250000).
    pub number: u64,
    /// Old-style handbook ID (e.g. "M0692 N0256").
    pub id: Option<String>,
    /// The sequence values.
    pub data: Vec<BigInt>,
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
    /// Keyword tags (e.g. [Nonn, Core, Nice, Hard]).
    pub keyword: Vec<Keyword>,
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
            keyword: parse_keywords(&e.keyword),
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
    #[serde(default)]
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
