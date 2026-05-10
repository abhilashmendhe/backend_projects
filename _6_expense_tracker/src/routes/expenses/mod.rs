use actix_web::{
    middleware::from_fn,
    web::{self, ServiceConfig, scope},
};

use crate::{
    middleware::require_auth::require_auth,
    routes::expenses::{create_expense::create_expense, get_all_expenses::get_all_expenses, get_expense_by_id::get_expense_by_id},
};

pub mod create_expense;
pub mod get_all_expenses;
pub mod get_expense_by_id;

pub fn expense_view_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/expenses")
            .wrap(from_fn(require_auth))
            .route("", web::post().to(create_expense))
            .route("", web::get().to(get_all_expenses))
            .route("/{id}", web::get().to(get_expense_by_id))
    );
}
