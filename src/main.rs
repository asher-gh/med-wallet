use axum::handler::Handler;
use medwallet_server::*;
use tower_http::limit::RequestBodyLimitLayer;

#[tokio::main]
async fn main() {
    //load .env
    dotenvy::dotenv().unwrap();

    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/docs/:id", get(serve_asset))
        .route(
            "/docs",
            post(upload_file.layer((
                // Replace the default body limit of 2MB with 10MB
                DefaultBodyLimit::disable(),
                RequestBodyLimitLayer::new(1024 * 10_000 /* ~10mb */),
            ))),
        )
        .with_state(pool);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!(
        "listening on {}, DATABASE_URL={}",
        address, db_connection_str
    );

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
