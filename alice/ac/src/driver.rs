use std::{
    io::{self, BufRead, BufReader, Write, stdin, stdout},
    path::{Path, PathBuf},
    str::FromStr,
};

use ac_db::db::AliceDatabaseTrait;
use ac_diag::{Diagnostic, DiagnosticKind};
use ac_interface::{Config, run_alice};
use ac_ir::source::SourceFile;
use ac_query::parse_file_query;
use ariadne::{FnCache, Label, Report, ReportKind};
use clap::Parser;
use thiserror::Error;

#[derive(Parser, Clone, Debug)]
#[command(version)]
pub struct AliceDriver {
    /// Input source file
    #[arg(default_value = "-")]
    input: AliceInput,
}

impl AliceDriver {
    pub fn run(self) {
        run_alice(Config::from(self), |db| match db.input() {
            Some(file) => Self::run_file(db, file),
            None => Self::run_repl(db),
        })
    }

    fn run_file(db: &dyn AliceDatabaseTrait, fpath: &Path) {
        let contents = std::fs::read_to_string(fpath).expect("unable to read file");
        let source_file = SourceFile::new(db, PathBuf::from(fpath), contents);

        let ast = parse_file_query::parse_file(db, source_file);

        // TODO: handle accumulators via strategy pattern?
        let diags = parse_file_query::parse_file::accumulated::<Diagnostic>(db, source_file);
        let mut report_cache = FnCache::new(|raw_path: &&str| -> Result<String, ReportError> {
            let path = PathBuf::from(raw_path);
            db.get_source_file(&path)
                .map(|file| file.contents(db).clone())
                .ok_or(ReportError::FileNotFound(path.clone()))
        });
        for diag in diags {
            let span = diag.span();
            let fpath = span.file(db).path(db).to_str().expect("invalid file path");

            Report::build(
                convert_diagnostic_kind(diag.kind),
                (fpath, span.start(db)..span.end(db)),
            )
            .with_message(diag.msg.clone())
            .with_label(Label::new((fpath, span.start(db)..span.end(db))))
            .finish()
            .eprint(&mut report_cache)
            .expect("unable to report diag");
        }
    }

    fn run_repl(db: &dyn AliceDatabaseTrait) {
        let input = stdin();
        let mut reader = BufReader::new(input);
        let mut line = String::new();
        let mut output = stdout();

        loop {
            write!(&mut output, "🦊 >>> ").expect("unable to write prompt invitation");
            output.flush().expect("unable to flush output writer");

            line.clear();

            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => (), // TODO: impl
                Err(_) => break,
            }
        }
    }
}

impl Default for AliceDriver {
    fn default() -> Self {
        AliceDriver::parse()
    }
}

impl From<AliceDriver> for Config {
    fn from(value: AliceDriver) -> Self {
        let input = match value.input {
            AliceInput::Stdin => None,
            AliceInput::File(path_buf) => Some(path_buf),
        };

        Self { input }
    }
}

#[derive(Clone, Debug)]
enum AliceInput {
    Stdin,
    File(PathBuf),
}

impl FromStr for AliceInput {
    type Err = AliceInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const STDIN_INPUT: &str = "-";
        match s {
            STDIN_INPUT => Ok(AliceInput::Stdin),
            s => {
                let fpath = PathBuf::from_str(s)
                    .unwrap()
                    .canonicalize()
                    .map_err(AliceInputError::InvalidInput)?;

                if fpath.is_file() {
                    Ok(AliceInput::File(fpath))
                } else {
                    Err(AliceInputError::NotSupportedInput(fpath))
                }
            }
        }
    }
}

#[derive(Error, Debug)]
enum AliceInputError {
    #[error("{0}")]
    InvalidInput(#[from] io::Error),
    #[error("unsupported input `{0}`")]
    NotSupportedInput(PathBuf),
}

fn convert_diagnostic_kind<'a>(kind: DiagnosticKind) -> ReportKind<'a> {
    use DiagnosticKind::*;
    match kind {
        Error => ReportKind::Error,
        Warning => ReportKind::Warning,
        Notice => ReportKind::Advice,
    }
}

#[derive(Error, Debug)]
enum ReportError {
    #[error("file not found {0}")]
    FileNotFound(PathBuf),
}
