use cotton::prelude::*;

// https://docs.rs/structopt/0.3.2/structopt/index.html#how-to-derivestructopt
/// Does stuff
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    logging: LoggingOpt,

    #[structopt(flatten)]
    dry_run: DryRunOpt,
}

fn main() -> FinalResult {
    let args = Cli::from_args();
    init_logger(&args.logging, vec![module_path!()]);

    warn!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
