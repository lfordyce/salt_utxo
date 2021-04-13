mod db;
mod entity;
mod handler;
mod state;
mod utils;

use crate::entity::repository::Repository;
use state::State;

pub type Request = tide::Request<State>;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    tide::log::with_level(tide::log::LevelFilter::Info);

    let repo = db::BtcRepo::new().await;
    let app = get_app(Box::new(repo)).await?;

    app.listen(format!("0.0.0.0:{}", port)).await?;
    Ok(())
}

pub async fn get_app(
    repository: Box<dyn Repository + Send + Sync + 'static>,
) -> tide::Result<tide::Server<State>> {
    let state = State::new(repository).await?;
    let mut app = tide::with_state(state);

    app.at("/all").get(handler::utxo::index);
    app.at("/utxo").post(handler::utxo::create_txn);
    app.at("/balance/:address").get(handler::utxo::get_balance);

    Ok(app)
}
