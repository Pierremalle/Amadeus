use axum::{Router, routing::get};

pub async fn paths() -> &'static str {
    r#"
-----------------------------------------------------------------------------------------------------------------------------------------
        PATH                |           SAMPLE COMMAND                                                                                  
-----------------------------------------------------------------------------------------------------------------------------------------
                            |
"#
}

pub fn add_routes(router: Router) -> Router {
    router.route("/", get(paths))
    // .route("/person/:id", personRoutes::create_person)
}
