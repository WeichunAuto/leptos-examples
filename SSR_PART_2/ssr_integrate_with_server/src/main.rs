#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use ssr_integrate_with_server::app::*;
    use tower_http::timeout::TimeoutLayer;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Initialize logging and tracing
    use ssr_integrate_with_server::my_config::tracing_init;
    tracing_init::init();
    tracing::info!("Starting the application server......");

    // request timeout, default 10s
    use axum::http::StatusCode;
    use std::time::Duration;
    let timeout = TimeoutLayer::with_status_code(
        StatusCode::REQUEST_TIMEOUT, // 408
        Duration::from_secs(10),
    );

    // request path, latency and status tracing. 
    use axum::extract::Request;
    use ssr_integrate_with_server::middleware::tracing_request::LatencyOnResponse;
    use tower_http::trace::TraceLayer;
    let tracing = TraceLayer::new_for_http()
        .make_span_with(|request: &Request| {
            let method = request.method();
            let path = request.uri().path();
            let id = xid::new(); // Generate unique request ID
            tracing::info_span!("Api Request: ", id = %id, method = %method, path = %path)
        })
        .on_request(())
        .on_failure(())
        .on_response(LatencyOnResponse);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(timeout)
        .layer(tracing)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
