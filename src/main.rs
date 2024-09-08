use std::{env, net::SocketAddr};

use askama::Template;
use axum::{extract::Query, response::Html, routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use axum_server_dual_protocol;
use dotenv::dotenv;
use memory_serve::{load_assets, MemoryServe};
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, Session, SessionManagerLayer};
mod types;
use types::{};

#[derive(Template)]
#[template(path = "main.html")]
struct MainPage {

}

#[derive(Template)]
#[template(path = "404.html")]
struct UnknownPage {

}

#[derive(Template)]
#[template(path = "sound.html")]
struct SoundPage {

}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutPage {

}

#[tokio::main]
async fn main() {
    dotenv().expect("expected dotenv");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(1000)));
    let memory_router = MemoryServe::new(load_assets!("public"))
        .into_router()
        .layer(session_layer.clone());

    let app = Router::new()
        .merge(memory_router)
        .route("/", get(show_main))
        .route("/sound", get(show_sound))
        .route("/about", get(show_about))
        .fallback(unknown_page)
        .layer(session_layer);
    let listener;
    if env::var("PROD_LUBBA").unwrap_or("".to_string()) == "" {
        listener = SocketAddr::from(([127, 0, 0, 1], 3000));
    } else {
        listener = SocketAddr::from(([0, 0, 0, 0], 443));
    }

    let config = RustlsConfig::from_pem_file("private/cert.pem", "private/key.pem")
        .await
        .unwrap();

    println!("listening on {}", listener);
    axum_server_dual_protocol::bind_dual_protocol(listener, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn show_main(session: Session) -> Html<String> {
    Html(
        MainPage {

        }
        .render()
        .unwrap(),
    )
}

async fn show_sound(session: Session) -> Html<String> {
    Html(
        SoundPage {

        }
        .render()
        .unwrap(),
    )
}

async fn show_about(session: Session) -> Html<String> {
    Html(
        AboutPage {

        }
        .render()
        .unwrap(),
    )
}

async fn unknown_page(session: Session) -> Html<String> {
    Html(
        UnknownPage {

        }
        .render()
        .unwrap(),
    )
}
