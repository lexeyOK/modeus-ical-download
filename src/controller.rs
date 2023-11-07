use anyhow::Result;
use thirtyfour::{
    common::capabilities::firefox::FirefoxPreferences, fantoccini::wd::Locator,
    prelude::ElementQueryable, By, DesiredCapabilities, FirefoxCapabilities, WebDriver,
};

pub fn make_capb(downloads_path: Option<&str>) -> Result<FirefoxCapabilities> {
    let mut capb = DesiredCapabilities::firefox();
    let mut firefox_preferences = FirefoxPreferences::new();
    firefox_preferences.set("browser.download.folderList", 2)?;
    if let Some(path) = downloads_path {
        firefox_preferences.set("browser.download.dir", path)?;
    }
    capb.set_preferences(firefox_preferences)?;
    //capb.add_firefox_option("args", ["-headless"])?;
    Ok(capb)
}

pub async fn auth_platform(c: &WebDriver, user_name: &str, password: &str) -> Result<()> {
    c.goto("https://tyuiu.modeus.org/schedule-calendar/my")
        .await?;

    // click on "Студенты" button
    c.find(By::Css("div.idp:nth-child(4)"))
        .await?
        .click()
        .await?;

    // fill the "loginForm" form
    let login_form = c.form(By::Id("loginForm")).await?;
    login_form.set_by_name("UserName", user_name).await?;
    login_form.set_by_name("Password", password).await?;
    login_form.submit_with(Locator::Id("submitButton")).await?;
    Ok(())
}

pub async fn perform_actions(c: &WebDriver) -> Result<()> {
    // save .ics file for the next week
    c.query(By::Css(".fc-next-button"))
        .first()
        .await?
        .click()
        .await?;
    dbg!(c.query(By::Css(".fc-event")).first().await?.text().await?);
    c.query(By::Css(".icon-icalendar"))
        .first()
        .await?
        .click()
        .await?;
    Ok(())
}
