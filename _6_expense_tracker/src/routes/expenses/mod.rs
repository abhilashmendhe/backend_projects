use actix_web::{middleware::from_fn, web::{self, ServiceConfig, scope}};

use crate::{middleware::require_auth::require_auth, routes::expenses::create_expense::create_expense};

pub mod create_expense;

pub fn expense_view_factory(app: &mut ServiceConfig) {

    app.
        service(
    scope("/expenses")
            .wrap(from_fn(require_auth))
            .route("", web::post().to(create_expense))
    );
}