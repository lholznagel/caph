mod database;
mod error;
mod tools;

pub use self::database::*;
pub use self::error::*;
pub use self::tools::*;

use indicatif::{ProgressBar, ProgressStyle};

pub fn new_progress_bar() -> ProgressBar {
    let progress_bar_style = ProgressStyle::default_spinner()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
        .template("{spinner:.blue} {wide_msg}");

    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(progress_bar_style.clone());
    progress_bar.enable_steady_tick(80);
    progress_bar
}

pub const DEFAULT_REGIONS: &'static [&'static str] = &[
    "Aridia",
    "Black Rise",
    "Derelik",
    "Devoid",
    "Domain",
    "Essence",
    "Everyshore",
    "Heimatar",
    "Kador",
    "Khanid",
    "Kor-Azor",
    "Lonetrek",
    "Metropolis",
    "Molden Heath",
    "Placid",
    "Sinq Laison",
    "Solitude",
    "Tash-Murkon",
    "The Bleak Lands",
    "The Citadel",
    "The Forge",
    "Verge Vendor",
];
