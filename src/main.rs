extern crate cotton;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;

use cotton::prelude::*;

// https://docs.rs/structopt/0.2.12/structopt/index.html#how-to-derivestructopt
/// Does stuff
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    logging: LoggingArgs,

    /// Just print what would have been done
    #[structopt(long = "dry-run",short = "-d")]
    dry_run: bool,
}

fn main() {
    let args = Cli::from_args();
    init_logger(&args.logging, module_path!());

    info!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}