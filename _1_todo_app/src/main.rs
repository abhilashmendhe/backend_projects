/*
To run:
$ cargo watch -q -c -w src/ -x run
*/

use _1_todo_app::router::run;


#[tokio::main]
async fn main() {

    run().await;    
}
