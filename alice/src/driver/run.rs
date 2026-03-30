use crate::{
    compiler::{Config, run_alice},
    driver::{AliceDriver, repl::repl},
};

impl AliceDriver {
    pub fn run(self) {
        run_alice(Config::from(self), |alice| match alice.input() {
            "-" => repl(alice),
            _ => todo!(),
        })
    }
}
