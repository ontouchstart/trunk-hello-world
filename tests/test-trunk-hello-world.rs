use anyhow::Result;

use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};

#[test]
fn test() -> Result<()> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.new_tab()?;
    tab.navigate_to("https://ontouchstart.github.io/trunk-hello-world")?;
    let png_data = tab
        .wait_for_element("body")?
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
    // Save the screenshot to disc
    std::fs::write("screenshot.png", png_data)?;
    Ok(())
}
