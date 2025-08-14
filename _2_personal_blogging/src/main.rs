/*
To run:
    $ cargo watch -q -c -w src/ -x run
*/

use _2_personal_blogging::utils::errors::BlogAppError;
use _2_personal_blogging::run;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), BlogAppError> {
    
    tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();

    dotenv::dotenv().ok();
    
    run().await?;
    Ok(())
}
