/*
To run:
    $ cargo watch -q -c -w src/ -x run
To generate orm:
    $ sea-orm-cli generate entity --database-url postgres://postgres:keyoarbcat@localhost:5433/postgres -o src/database
*/

use _1_todo_app::{run, utils::app_state::AppState};
use dotenv::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    
    let db = match Database::connect(db_url).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("Failed to connect to the database: {:?}",err);
            panic!()
        }
    };

    let app_state = AppState {
        db,
        jwt_secret
    };

    run(
        app_state
    ).await;    
}
