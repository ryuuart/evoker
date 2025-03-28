use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Arguments for compilation and watching.
#[derive(Debug, Clone)]
pub struct CompileArgs {
    /// Path to input Typst file. Use `-` to read input from stdin.
    pub input: Input,

    /// Path to output file
    pub output: Option<Output>,

    /// World arguments.
    pub world: WorldArgs,

    /// The PPI (pixels per inch) to use for PNG export.
    pub ppi: f32,

    /// File path to which a Makefile with the current compilation's
    /// dependencies will be written.
    pub make_deps: Option<PathBuf>,

    /// Processing arguments.
    pub process: ProcessArgs,
}

/// Arguments for the construction of a world. Shared by compile, watch, and
/// query.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WorldArgs {
    /// Configures the project root (for absolute paths).
    pub root: Option<PathBuf>,

    /// Add a string key-value pair visible through `sys.inputs`.
    pub inputs: Vec<(String, String)>,

    /// Common font arguments.
    pub font: FontArgs,

    /// Arguments related to storage of packages in the system.
    pub package: PackageArgs,
}

/// Arguments for configuration the process of compilation itself.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ProcessArgs {
    /// Number of parallel jobs spawned during compilation. Defaults to number
    /// of CPUs. Setting it to 1 disables parallelism.
    pub jobs: Option<usize>,

    /// Enables in-development features that may be changed or removed at any
    /// time.
    pub features: Vec<Feature>,

    /// The format to emit diagnostics in.
    pub diagnostic_format: DiagnosticFormat,
}

/// Arguments related to where packages are stored in the system.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PackageArgs {
    /// Custom path to local packages, defaults to system-dependent location.
    pub package_path: Option<PathBuf>,

    /// Custom path to package cache, defaults to system-dependent location.
    pub package_cache_path: Option<PathBuf>,
}

/// Common arguments to customize available fonts.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FontArgs {
    /// Adds additional directories that are recursively searched for fonts.
    ///
    /// If multiple paths are specified, they are separated by the system's path
    /// separator (`:` on Unix-like systems and `;` on Windows).
    pub font_paths: Vec<PathBuf>,

    /// Ensures system fonts won't be searched, unless explicitly included via
    /// `--font-path`.
    pub ignore_system_fonts: bool,
}

/// An input that is either stdin or a real path.
#[derive(Debug, Clone, Deserialize)]
pub enum Input {
    /// Stdin, represented by `-`.
    Stdin,
    /// A non-empty path.
    Path(PathBuf),
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Input::Stdin => f.pad("stdin"),
            Input::Path(path) => path.display().fmt(f),
        }
    }
}

/// An output that is either stdout or a real path.
#[derive(Debug, Clone)]
pub enum Output {
    /// Stdout, represented by `-`.
    Stdout,
    /// A non-empty path.
    Path(PathBuf),
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Output::Stdout => f.pad("stdout"),
            Output::Path(path) => path.display().fmt(f),
        }
    }
}

/// Which format to use for the generated output file.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum OutputFormat {
    Pdf,
    Png,
    Svg,
    Html,
}

/// Which format to use for diagnostics.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize)]
pub enum DiagnosticFormat {
    #[default]
    Human,
    Short,
}

/// An in-development feature that may be changed or removed at any time.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
pub enum Feature {
    Html,
}

// Output file format for query command
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum SerializationFormat {
    #[default]
    Json,
    Yaml,
}

/// Implements parsing of page ranges (`1-3`, `4`, `5-`, `-2`), used by the
/// `CompileCommand.pages` argument, through the `FromStr` trait instead of a
/// value parser, in order to generate better errors.
///
/// See also: https://github.com/clap-rs/clap/issues/5065
#[derive(Debug, Clone)]
pub struct Pages(pub RangeInclusive<Option<NonZeroUsize>>);

impl FromStr for Pages {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value
            .split('-')
            .map(str::trim)
            .collect::<Vec<_>>()
            .as_slice()
        {
            [] | [""] => Err("page export range must not be empty"),
            [single_page] => {
                let page_number = parse_page_number(single_page)?;
                Ok(Pages(Some(page_number)..=Some(page_number)))
            }
            ["", ""] => Err("page export range must have start or end"),
            [start, ""] => Ok(Pages(Some(parse_page_number(start)?)..=None)),
            ["", end] => Ok(Pages(None..=Some(parse_page_number(end)?))),
            [start, end] => {
                let start = parse_page_number(start)?;
                let end = parse_page_number(end)?;
                if start > end {
                    Err("page export range must end at a page after the start")
                } else {
                    Ok(Pages(Some(start)..=Some(end)))
                }
            }
            [_, _, _, ..] => Err("page export range must have a single hyphen"),
        }
    }
}

/// Parses a single page number.
fn parse_page_number(value: &str) -> Result<NonZeroUsize, &'static str> {
    if value == "0" {
        Err("page numbers start at one")
    } else {
        NonZeroUsize::from_str(value).map_err(|_| "not a valid page number")
    }
}

/// Parses key/value pairs split by the first equal sign.
///
/// This function will return an error if the argument contains no equals sign
/// or contains the key (before the equals sign) is empty.
fn parse_sys_input_pair(raw: &str) -> Result<(String, String), String> {
    let (key, val) = raw
        .split_once('=')
        .ok_or("input must be a key and a value separated by an equal sign")?;
    let key = key.trim().to_owned();
    if key.is_empty() {
        return Err("the key was missing or empty".to_owned());
    }
    let val = val.trim().to_owned();
    Ok((key, val))
}

/// Parses a UNIX timestamp according to <https://reproducible-builds.org/specs/source-date-epoch/>
fn parse_source_date_epoch(raw: &str) -> Result<DateTime<Utc>, String> {
    let timestamp: i64 = raw
        .parse()
        .map_err(|err| format!("timestamp must be decimal integer ({err})"))?;
    DateTime::from_timestamp(timestamp, 0).ok_or_else(|| "timestamp out of range".to_string())
}
