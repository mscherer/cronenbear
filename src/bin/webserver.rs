use askama::Template;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum_response_cache::CacheLayer;
use cronenbear::aliases::AliasID;
use cronenbear::aliases::Aliases;
use cronenbear::country_calendar::CountryCalendar;
use cronenbear::index_page::IndexTemplate;
use cronenbear::merged_calendar::MergedCalendar;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Clone, Debug)]
pub struct AppState {
    aliases: Aliases,
    all_merged_calendars: Arc<HashMap<AliasID, MergedCalendar>>,
}

pub async fn ical_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Some(calendar_name) = id.strip_suffix(".ics")
        && let Some(merged) = state.all_merged_calendars.get(&calendar_name.into())
    {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "text/calendar".parse().unwrap());
        return (headers, merged.generate_ical().to_string()).into_response();
    }

    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

pub async fn health_checker_handler() -> impl IntoResponse {
    "All is fine"
}

pub async fn index_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = IndexTemplate::new(state.aliases.get_all_aliases_named());
    if let Ok(body) = template.render() {
        (StatusCode::OK, Html(body)).into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

const PORT: u16 = 1107;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Loading calendars");
    let aliases = Aliases::load_hardcoded();
    let mut all_calendars = HashMap::new();
    for c in aliases.get_all_calendars_to_create() {
        all_calendars.insert(c.clone(), CountryCalendar::try_from(c.as_str()).unwrap());
    }

    let mut all_merged_calendars = HashMap::new();
    for a in aliases.get_all_aliases() {
        let formating = aliases.get_formatting(&a);
        let name = String::from(aliases.get_name(&a).unwrap());
        let mut m = MergedCalendar::new(&name);
        if let Some(members) = aliases.get_members(&a) {
            for c in members {
                m.add(all_calendars.get(&c).unwrap(), &formating)
            }
        }
        all_merged_calendars.insert(a.clone(), m);
    }

    let app_state = AppState {
        aliases,
        all_merged_calendars: Arc::new(all_merged_calendars),
    };

    // TODO faire un cache des calendriers en local
    //
    let port: u16 = match env::var("PORT") {
        Ok(val) => val.parse().unwrap_or_else(|_| {
            tracing::warn!("Incorrect PORT value: {val}, using default: {PORT}");
            PORT
        }),
        Err(_) => PORT,
    };
    let app = Router::new()
        .route("/", get(index_handler))
        // not supported until 0.9
        //.route("/calendar/{id}.ics", get(ical_handler));
        .route(
            "/calendar/{id}",
            get(ical_handler).layer(CacheLayer::with_lifespan(24 * 60 * 60)),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        // https://github.com/tokio-rs/axum/discussions/355
        .route("/healthz", get(health_checker_handler))
        .with_state(app_state);

    tracing::info!("Server started on port {port}");

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
