use crate::{
    driver::AliceDriver,
    frontend::{Alice, Config, run_alice},
};
use std::{
    io::{BufRead, BufReader, Write, stdin, stdout},
    path::Path,
};

impl AliceDriver {
    pub fn run(self) {
        run_alice(Config::from(self), |alice| match alice.input() {
            Some(file) => Self::run_file(alice, file),
            None => Self::run_repl(alice),
        })
    }

    fn run_file(alice: &Alice, fpath: &Path) {
        let contents = std::fs::read_to_string(fpath).expect("unable to read file");
    }

    fn run_repl(alice: &Alice) {
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
