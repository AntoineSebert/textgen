use clap::{crate_authors, crate_version, AppSettings, Clap};
use textgen::generate;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Input for the program
    #[clap(name = "input", required = true)]
    input: String,
    /// Inline, the input is used as corpus
    #[clap(short, long, name = "inline")]
    inline: bool,
    /// Sets the length of the keys in chars
    #[clap(short, long, default_value = "3", name = "key_length")]
    key_length: u8,
    /// Sets the length of the values in chars
    #[clap(short, long, default_value = "3", name = "value_length")]
    value_length: u8,
    /// The adjacency matrix source is the output
    #[clap(short, long, name = "source", conflicts_with_all = &["token_mode", "count"], )]
    source: bool,
    /// Count of entities (sentences or tokens) in the output text
    #[clap(
        short,
        long,
        default_value = "5",
        name = "count",
        conflicts_with = "source"
    )]
    count: u8,
    /// In token mode, the program generates tokens until count.
    /// If this option is not present, the program defaults to Sentence mode
    /// In this mode, the program will try to generate sentences starting by a capitalized letter (if any), and ending by a dot
    #[clap(short, long, name = "token_mode", conflicts_with = "source")]
    token_mode: bool,
}

///
/// ```
/// ```
#[doc(hidden)]
fn main() -> Result<(), String> {
    let opts: Opts = Opts::parse();

    generate(
        opts.input,
        opts.inline,
        opts.key_length,
        opts.value_length,
        opts.count.into(),
        opts.token_mode,
        opts.source,
    )
}
