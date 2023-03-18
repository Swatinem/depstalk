use std::cmp::Reverse;

use clap::Parser;
use extractor::Dependent;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

use crate::fetcher::Fetcher;
use crate::loader::Loader;

mod extractor;
mod fetcher;
mod loader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The full repo to scrape ($owner/$repo)
    repo: String,

    /// Output the top N dependents by stars
    #[arg(short = 'n', long, default_value_t = 10)]
    top: usize,

    /// Output list as JSON
    #[arg(short, long)]
    json: bool,

    /// Pages to scrape
    #[arg(short, long, default_value_t = 20)]
    scrape: usize,

    /// Do a fresh fetch instead of reusing existing state
    #[arg(short, long)]
    fresh: bool,

    /// Verbose log output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        let filter = EnvFilter::from_default_env().add_directive("depstalk=debug".parse()?);
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_span_events(FmtSpan::CLOSE)
            .init();
    }

    let cwd = std::env::current_dir()?;
    let mut loader = if cli.fresh {
        Loader::new(&cwd, &cli.repo)
    } else {
        Loader::load_or_create(&cwd, &cli.repo)?
    };

    let fetcher = Fetcher::new()?;
    let fetch_res = fetcher.fetch_pages(&mut loader.state, cli.scrape);
    let dependents = &mut loader.state.dependents;
    // lets always sort this, because why not…
    // oh dear borrow checker, why can’t I just return `&d.owner` from the `by_key` closure?
    fn sort_key(dep: &Dependent) -> (Reverse<usize>, &str, &str) {
        (Reverse(dep.stars), &dep.owner, &dep.repo)
    }
    dependents.sort_unstable_by(|a, b| sort_key(a).cmp(&sort_key(b)));
    //dependents.sort_unstable_by_key(|d| (Reverse(d.stars), &d.owner));

    // always print the output and persist the state before propagating fetcher errors
    if cli.json {
        serde_json::to_writer_pretty(std::io::stdout(), dependents)?;
    } else {
        let len = dependents.len();
        let n = cli.top.min(len);
        println!(
            "Top {n} dependents of `{}` (out of {len} scraped):",
            cli.repo,
        );
        for dep in dependents.iter().take(n) {
            println!("- {}/{} ({} stars)", dep.owner, dep.repo, dep.stars);
        }
    }
    loader.store()?;

    fetch_res?;

    Ok(())
}
