use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::register::register_handler,
    ),
    components(
        schemas(
            crate::models::user::RegisterUser,
            common::models::user::User,
        )
    ),
    tags(
        (name = "authentication", description = "User registration endpoint")
    ),
    info(
        title = "Rust Auth API",
        version = "0.1.0",
        description = "API Documentation"
    ))]
pub struct ApiDoc;
