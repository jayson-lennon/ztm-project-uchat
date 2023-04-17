use axum::{
    extract::DefaultBodyLimit,
    http::HeaderValue,
    routing::{get, post},
    Router,
};
use hyper::{header::CONTENT_TYPE, Method};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use uchat_endpoint::{
    post::endpoint::{
        Bookmark, BookmarkedPosts, Boost, HomePosts, LikedPosts, NewPost, React, TrendingPosts,
        Vote,
    },
    user::endpoint::{CreateUser, Login},
    Endpoint,
};

use crate::{
    handler::{self, with_handler, with_public_handler},
    AppState,
};

pub fn new_router(state: AppState) -> axum::Router {
    let img_route = {
        use uchat_endpoint::app_url::user_content;
        format!("{}{}", user_content::ROOT, user_content::IMAGES)
    };

    let public_routes = Router::new()
        .route("/", get(move || async { "this is the root page" }))
        .route(&format!("/{img_route}:id"), get(handler::load_image))
        .route(CreateUser::URL, post(with_public_handler::<CreateUser>))
        .route(Login::URL, post(with_public_handler::<Login>));

    let authorized_routes = Router::new()
        .route(NewPost::URL, post(with_handler::<NewPost>))
        .route(Bookmark::URL, post(with_handler::<Bookmark>))
        .route(Boost::URL, post(with_handler::<Boost>))
        .route(Vote::URL, post(with_handler::<Vote>))
        .route(React::URL, post(with_handler::<React>))
        .route(TrendingPosts::URL, post(with_handler::<TrendingPosts>))
        .route(HomePosts::URL, post(with_handler::<HomePosts>))
        .route(LikedPosts::URL, post(with_handler::<LikedPosts>))
        .route(BookmarkedPosts::URL, post(with_handler::<BookmarkedPosts>))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(8 * 1024 * 1024));

    Router::new()
        .merge(public_routes)
        .merge(authorized_routes)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Micros),
                        ),
                )
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                        .allow_credentials(true)
                        .allow_origin(
                            std::env::var("FRONTEND_URL")
                                .unwrap()
                                .parse::<HeaderValue>()
                                .unwrap(),
                        )
                        .allow_headers([CONTENT_TYPE]),
                )
                .layer(axum::Extension(state.clone())),
        )
        .with_state(state)
}
