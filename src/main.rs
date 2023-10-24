mod api;
mod indicators;
mod models;
mod plotter;
mod config;



#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the mr.CryptoRobot, yopta (▼_▼ )");

    if let Err(bot_error) = api::telegram::run_robot().await {
	log::error!("{:#?}", bot_error);
    }
}
