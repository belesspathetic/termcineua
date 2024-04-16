use serde_json::json;
use std::process::{Child, Command};
use thirtyfour::prelude::*;

pub struct Browser {
    pub driver: WebDriver,
    pub command: Child,
}

pub async fn start() -> anyhow::Result<Browser> {
    let command = Command::new("chromedriver").spawn()?;

    async_wait_n_sec(3).await?;
    let mut caps = DesiredCapabilities::chrome();
    caps.insert(
        "goog:chromeOptions".to_string(),
        json!({
            "args": [
                "--disable-blink-features=AutomationControlled",
                "--disable-notifications",
                "--disable-popup-blocking",
                "--disable-infobars",
                "--disable-dev-shm-usage",
                "--disable-gpu",
                "--no-sandbox",
                "--disable-extensions",
                "--disable-web-security",
                "--disable-setuid-sandbox",
                "--disable-ipc-flooding",
                "--disable-background-networking",
                "--disable-background-timer-throttling",
                "--disable-breakpad",
                "--disable-client-side-phishing-detection",
                "--disable-component-extensions-with-background-pages",
                "--disable-default-apps",
                "--disable-hang-monitor",
                "--disable-logging",
                "--disable-sync",
                "--metrics-recording-only",
                "--no-first-run",
                "--mute-audio",
                "--start-maximized"
            ],
            "prefs": {
                "profile.managed_default_content_settings.notifications": 1
            }
        }),
    );
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let browser = Browser {
        driver: driver,
        command: command,
    };

    Ok(browser)
}

pub async fn close(browser: Browser) -> anyhow::Result<()> {
    let driver = browser.driver;
    driver.quit().await?;

    let mut command = browser.command;
    command.kill()?;
    async_wait_n_sec(2).await?;

    Ok(())
}

pub async fn async_wait_n_sec(n: u64) -> anyhow::Result<()> {
    let delay = tokio::time::Duration::from_secs(n);
    tokio::time::sleep(delay).await;

    Ok(())
}
