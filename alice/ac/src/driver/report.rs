use std::path::PathBuf;

use ac_db::db::AcDbTrait;
use ac_diag::{Diagnostic, DiagnosticKind};
use ariadne::{FnCache, Label, Report, ReportKind};
use thiserror::Error;

pub(super) fn report<'db>(
    db: &'db dyn AcDbTrait,
    diags: impl IntoIterator<Item = &'db Diagnostic>,
) {
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

fn convert_diagnostic_kind<'a>(kind: DiagnosticKind) -> ReportKind<'a> {
    match kind {
        DiagnosticKind::Error => ReportKind::Error,
        DiagnosticKind::Warning => ReportKind::Warning,
        DiagnosticKind::Notice => ReportKind::Advice,
    }
}

#[derive(Error, Debug)]
enum ReportError {
    #[error("file not found {0}")]
    FileNotFound(PathBuf),
}
