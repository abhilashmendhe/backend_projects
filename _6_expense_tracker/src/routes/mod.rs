use actix_web::web::ServiceConfig;

use crate::routes::{expenses::expense_view_factory, users::users_view_factory};

pub mod users;
pub mod expenses;

pub fn sub_views_factory(app: &mut ServiceConfig) {
    app
        .configure(users_view_factory)
        .configure(expense_view_factory);
}