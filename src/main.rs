use brush::app::APP;

#[tokio::main]
async fn main() {
    let mut terminal = ratatui::init();
    let mut app = APP::new().await;
    app.run(&mut terminal).unwrap();

    ratatui::restore();
}
