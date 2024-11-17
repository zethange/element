use axum::Extension;
use sea_orm::{ConnectOptions, Database};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod entities;
mod user;

const USERS_TAG: &str = "users";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = USERS_TAG, description = "users managment api tools")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let connection_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://demo:demo@localhost:5432/demo?sslmode=disable".to_string());

    let mut opt = ConnectOptions::new(connection_url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true);

    let db = Database::connect(opt).await.unwrap();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/users", user::router::get_router())
        .split_for_parts();

    let app = router
        .nest("/users", user::html_router::get_html_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api))
        .layer(Extension(db));

    let url = format!("0.0.0.0:{}", port);
    println!("server started on {}", url);

    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
