use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};

fn main() {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .sandbox(false)
            .build()
            .expect("Could not find chrome-executable"),
    )
    .expect("Could not set browser");
    let tab = browser.new_tab().expect("Could not create tab");
    let url = "https://ontouchstart.github.io/trunk-hello-world".to_string();
    tab.navigate_to(&url).expect("Could not navigate to url");
    let h1 = tab.wait_for_element("h1").expect("Could not get h1");
    assert_eq!(
        h1.get_content().unwrap(),
        "<h1>Hello, world from Vanilla Rust!</h1>"
    );
    let png_data = tab
        .wait_for_element("body")
        .expect("Could not get body")
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)
        .expect("Cound not genearte PNG");
    std::fs::write("screenshot.png", png_data).expect("Could not save png");
}
