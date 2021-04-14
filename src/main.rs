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

    let mut app = tide::Server::with_state(state.clone());
    app.at("/api/v1").nest({
        let mut api = tide::Server::with_state(state.clone());

        api.at("/addrs").get(handler::utxo::get_balance); // TODO

        api.at("/addrs/:address").get(handler::utxo::get_balance);

        api.at("/records/offset/:offset/limit/:limit")
            .get(handler::utxo::index);

        api.at("/utxo").post(handler::utxo::create_txn); // TODO
        api
    });
    Ok(app)
}
