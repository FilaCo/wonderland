use std::{
    io::{BufRead, BufReader, Write, stdin, stdout},
    path::{Path, PathBuf},
};

use ac_db::db::AcDbTrait;
use ac_diag::Diagnostic;
use ac_interface::{Config, run_alice};
use ac_ir::source::SourceFile;
use ac_query::parse_file_query;

use crate::driver::{AliceDriver, report::report};

impl AliceDriver {
    pub fn run(self) {
        run_alice(Config::from(self), |db| match db.input() {
            Some(file) => Self::run_file(db, file),
            None => Self::run_repl(db),
        })
    }

    fn run_file(db: &dyn AcDbTrait, fpath: &Path) {
        let contents = std::fs::read_to_string(fpath).expect("unable to read file");
        let source_file = SourceFile::new(db, PathBuf::from(fpath), contents);

        let ast = parse_file_query::parse_file(db, source_file);
        let diags = parse_file_query::parse_file::accumulated::<Diagnostic>(db, source_file);
        report(db, diags);
    }

    fn run_repl(db: &dyn AcDbTrait) {
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
