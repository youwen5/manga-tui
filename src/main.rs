#![forbid(unsafe_code)]
use once_cell::sync::Lazy;
use ratatui::backend::CrosstermBackend;
use ratatui_image::picker::Picker;
use reqwest::Client;

use self::backend::build_data_dir;
use self::backend::error_log::init_error_hooks;
use self::backend::fetch::{MangadexClient, MANGADEX_CLIENT_INSTANCE};
use self::backend::tui::{init, restore, run_app};

mod utils;

mod backend;
/// These would be like the frontend
mod view;
mod filter;

pub static PICKER: Lazy<Option<Picker>> = Lazy::new(|| {
    let maybe_picker = Picker::from_termios();
    match maybe_picker {
        Ok(mut picker) => {
            picker.guess_protocol();
            Some(picker)
        }
        Err(_) => None,
    }
});

#[tokio::main(flavor = "multi_thread", worker_threads = 7)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match build_data_dir() {
        Ok(_) => {}
        Err(e) => {
            panic!("Data dir could not be created, details : {e}")
        }
    }

    let user_agent = format!(
        "manga-tui/0.beta-testing1.0 ({}/{}/{})",
        std::env::consts::FAMILY,
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    let mangadex_client =
        MangadexClient::new(Client::builder().user_agent(user_agent).build().unwrap());

    MANGADEX_CLIENT_INSTANCE.set(mangadex_client).unwrap();

    init_error_hooks()?;
    init()?;
    run_app(CrosstermBackend::new(std::io::stdout())).await?;
    restore()?;
    Ok(())
}
