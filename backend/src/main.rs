use dotenv;
use::std::env as std_env;

use sqlx::Pool;
use sqlx::PgPool;
use tide::http::headers::HeaderValue;
use tide::security::CorsMiddleware;
use tide::security::Origin;
use tide::Server;

#[cfg(test)]
mod tests;

mod endpoints;
mod env;
mod middlewares;
mod responses;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::try_init().ok();

    let db_pool = make_db_pool().await;
    let app = server(db_pool).await;

    app.listen("127.0.0.1:8080").await.unwrap();
}

pub async fn make_db_pool() -> PgPool {
    let db_url = std_env::var("DATABASE_URL").unwrap();
    Pool::new(&db_url).await.unwrap()
}

async fn server(db_pool: PgPool) -> Server<State> {
    let mut server: Server<State> = Server::with_state(State { db_pool });
    
    server.with(
        CorsMiddleware::new()
            .allow_methods(
                "GET, POST, PUT, PATCH, DELETE, OPTIONS"
                    .parse::<HeaderValue>()
                    .unwrap(),
            )
            .allow_origin(Origin::Any),
    );
    server.with(middlewares::ErrorReponseToJson);

    server.at("/users").post(endpoints::users::create);
    server
        .at("/users/:username/session")
        .post(endpoints::users::login)
        .delete(endpoints::users::logout);
    server
        .at("/users/:username/follow")
        .post(endpoints::users::follow);
    server
        .at("/users/:username/following")
        .get(endpoints::users::following);
    server
        .at("/users/:username/followers")
        .get(endpoints::users::followers);
    server.at("/users/:username").get(endpoints::users::get);

    server.at("/me").get(endpoints::me::get);
    server.at("/me/timeline").get(endpoints::me::timeline);

    server.at("/tweets").post(endpoints::tweets::create);

    server
}

#[derive(Debug, Clone)]
pub struct State {
    db_pool: PgPool,
}
