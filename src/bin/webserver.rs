use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Html;
use axum::http::StatusCode;
use askama::Template;
use axum::routing::get;
use cronenbear::aliases::Aliases;
use cronenbear::country_calendar::CountryCalendar;
use cronenbear::google_public_calendar::GooglePublicCalendar;
use cronenbear::merged_calendar::MergedCalendar;
use cronenbear::religion_calendar::{ReligionCalendar, ReligionCode};
use cronenbear::index_page::IndexTemplate;

use std::env;

#[derive(Clone, Debug)]
pub struct AppState {
    aliases: Aliases
}

pub async fn ical_handler(
    State(sate): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // TODO parser le chemin et voir si il y a un .ics à la fin
    "ok"
}

pub async fn health_checker_handler() -> impl IntoResponse {
    "All is fine"
}

pub async fn index_handler(State(state): State<AppState>) -> impl IntoResponse {
    let calendars = state.aliases.get_all_aliases();
    let template = IndexTemplate::new(calendars);
    if let Ok(body) = template.render() {
        (StatusCode::OK, Html(body)).into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

const PORT: u16 = 1107;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let aliases = Aliases::load_hardcoded();
    let app_state = AppState {
        aliases: aliases
    };

    // TODO
    // faire un calendrier par item dans un alias
    // faire un merged calendar par aliases
    // mettre ça dans une gigantesque hashmap
    // passer la hashmap en state

    // TODO faire un cache des calendriers en local
    //
    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse().unwrap_or_else(|_| {
            println!("Incorrect PORT value: {}, using default: {}", val, PORT);
            PORT
        }),
        Err(_) => PORT,
    };

    let app = Router::new()
        .route("/healthz", get(health_checker_handler))
        .route("/", get(index_handler))
        // not supported until 0.9
        //.route("/calendar/{id}.ics", get(ical_handler));
        .route("/calendar/{id}", get(ical_handler))
        .with_state(app_state);

    println!("Server started on port {}", port);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
