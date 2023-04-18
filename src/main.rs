use cotton::prelude::*;

/// Hello world!
#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    logging: ArgsLogger,

    #[command(flatten)]
    dry_run: ArgsDryRun,
}

fn main() -> FinalResult {
    let Cli {
        logging,
        dry_run,
    } = Cli::parse();
    setup_logger(logging, vec![module_path!()]);

    if !dry_run.enabled {
        warn!("Hello world!");
    }

    Ok(())
}

// vim: ft=rust
