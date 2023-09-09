use super::ServerState;
use axum::{
    extract::Path, extract::Query, extract::RawQuery, extract::State, response::AppendHeaders,
    response::Html, response::Redirect,
};
use std::{collections::HashMap, sync::Arc};
use tera::Context;

pub async fn search(
    state: State<Arc<ServerState>>,
    params: Query<Vec<(String, String)>>,
    RawQuery(query_string): RawQuery,
) -> (AppendHeaders<[(&'static str, String); 1]>, Html<String>) {
    let actors: Vec<String> = params
        .iter()
        .filter(|(k, _)| k == "actors")
        .map(|(_, v)| v)
        .filter(|v| !v.is_empty())
        .map(|v| v.trim().to_string())
        .collect();

    let res = state.db.search_movies(&actors).await;

    let mut ctx = Context::new();
    ctx.insert("actors", &actors);
    ctx.insert("results", &res);

    let template = state.tera.render("results.html", &ctx);

    let headers = match actors.len() {
        0 => AppendHeaders([("HX-Push-Url", String::from("/"))]),
        _ => AppendHeaders([(
            "HX-Push-Url",
            format!("/search?{}", query_string.expect("No query string?")),
        )]),
    };

    (headers, Html(template.unwrap()))
}

pub async fn suggestions(
    state: State<Arc<ServerState>>,
    params: Query<HashMap<String, String>>,
) -> Html<String> {
    let query = params.get("actors").unwrap();

    let suggestions = state.db.get_actor_suggestions(query.to_string()).await;

    let mut ctx = Context::new();
    ctx.insert("suggestions", &suggestions);

    let template = state.tera.render("suggestions.html", &ctx);
    Html(template.unwrap())
}

pub async fn poster(state: State<Arc<ServerState>>, Path(tconst): Path<String>) -> Redirect {
    if let Some(url) = state.cache.get(&tconst).await {
        return Redirect::to(&url);
    }

    let omdb_key = std::env::var("OMDB_KEY").expect("OMDB_KEY must be set!");
    let url = format!("http://www.omdbapi.com/?i={}&apikey={}", tconst, omdb_key);
    let response = reqwest::get(&url).await;
    let Ok(response) = response else {
        panic!("Error from omdb API: {:?}", response);
    };

    let json = response.json::<serde_json::Value>().await;
    let Ok(json) = json else {
        panic!("Error parsing JSON: {:?}", json);
    };

    let poster_url = json.get("Poster").and_then(|v| v.as_str());

    let redir = match poster_url {
        Some(url) if url != "N/A" => url,
        _ => "/static/placeholder.jpg",
    };

    state.cache.set(&tconst, &redir.to_string()).await;

    Redirect::to(redir)
}
