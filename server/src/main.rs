use axum::{
    extract::{Query, RawQuery, State},
    response::Html,
    routing::get,
    Router,
};
use poster_cache::PosterCache;
use std::sync::Arc;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
extern crate dotenv;

mod api_handlers;
mod database;
mod poster_cache;

pub struct ServerState {
    tera: Tera,
    db: database::DB,
    cache: poster_cache::PosterCache,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let tera = Tera::new("static/templates/*.html").expect("Error Parsing Templates");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let state = Arc::new(ServerState {
        tera,
        db: database::DB::new(db_url).await,
        cache: PosterCache::new(&redis_url).await,
    });

    let api_router = Router::new()
        .route("/search", get(api_handlers::search))
        .route("/suggest", get(api_handlers::suggestions))
        .route("/poster/:tconst", get(api_handlers::poster));

    let app = Router::new()
        .route("/", get(root))
        .route("/search", get(search))
        .nest("/api", api_router)
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let port = match std::env::var("SERVER_PORT") {
        Ok(port) => port,
        Err(_) => "5000".to_string(),
    };

    let bind_address = format!("0.0.0.0:{}", port);
    axum::Server::bind(&bind_address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(state: State<Arc<ServerState>>) -> Html<String> {
    let ctx = Context::new();
    let template = state.tera.render("index.html", &ctx);
    Html(template.unwrap())
}

async fn search(
    state: State<Arc<ServerState>>,
    params: Query<Vec<(String, String)>>,
    raw_query: RawQuery,
) -> Html<String> {
    let (_, Html(result_content)) = api_handlers::search(state.clone(), params, raw_query).await;

    let mut index_ctx = Context::new();
    index_ctx.insert("result_content", &result_content);
    let template = state.tera.render("index.html", &index_ctx);

    Html(template.unwrap())
}
