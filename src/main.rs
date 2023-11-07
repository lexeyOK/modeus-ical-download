mod controller;
use controller::*;

use anyhow::Result;
use std::{env::var, fs, path::PathBuf};
use thirtyfour::WebDriver;

#[tokio::main]
async fn main() -> Result<()> {
    let user_name = dbg!(var("MODEUS_USERNAME")).expect("Username provided");
    let password = var("MODEUS_PASSWORD").expect("Password provided");
    let downloads_path = var("MODEUS_DOWNLOADS_PATH").ok();

    let c = WebDriver::new(
        "http://localhost:4444",
        make_capb(downloads_path.as_deref())?,
    )
    .await?;
    auth_platform(&c, &user_name, &password).await?;
    perform_actions(&c).await?;
    c.quit().await?;

    if let Some(downloads_path) = downloads_path {
        let path = PathBuf::from(downloads_path);
        if path.is_dir() {
            for dir in fs::read_dir(path)? {
                let dir = dir?;
                if dir.path().is_file() && dir.path().ends_with("ics") {
                    println!("{:?}", dir.path());
                }
            }
        }
    }
    //let file = File::open(downloads_path+".ics")
    Ok(())
}
