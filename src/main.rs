use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};

fn main() {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    ).expect("Could not set browser");
    let tab = browser.new_tab().expect("Could not create tab");
    let url = "https://ontouchstart.github.io/trunk-hello-world".to_string();
    tab.navigate_to(&url).expect("Could not navigate to url");
    let png_data = tab
        .wait_for_element("body").expect("Could not get body")
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png).expect("Cound not genearte PNG");
    std::fs::write("screenshot.png", png_data).expect("Could not save png");
}
