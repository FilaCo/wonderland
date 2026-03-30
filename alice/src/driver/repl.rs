use std::io::{BufRead, BufReader, Write, stdin, stdout};

use crate::compiler::Alice;

pub(crate) fn repl(alice: &Alice) {
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
