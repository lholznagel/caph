use clap::{crate_authors, crate_version, Clap};
use caph_eve_online_cli::*;
use std::sync::{Arc, Mutex};

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Sell(SellCli),
    Buy,
    Misc,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let database = Arc::new(Mutex::new(Database::default()));

    match opts.subcmd {
        SubCommand::Sell(x) => match x.subcmd {
            Some(SellSubcommand::Ore(y)) => {
                SellOre::new(database.clone())
                    .collect_and_print(y.include, x.entries)
                    .await?
            }
            _ => {
                SellItem::new(database.clone())
                    .collect_and_print(x.items, x.entries, x.regions)
                    .await?
            }
        },
        SubCommand::Buy => {
            BuyItem::new(database.clone())
                .collect_and_print("Omber".into(), None, None)
                .await?;
        }
        SubCommand::Misc => {
            Misc::new(database.clone()).collect_and_print().await?;
        }
    }

    Ok(())
}
