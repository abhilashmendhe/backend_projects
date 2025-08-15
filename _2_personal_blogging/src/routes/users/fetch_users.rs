pub async fn get_all_users() {
    // let query = format!("SELECT * FROM users where id=1");
    // let row = match sqlx::query(&query).fetch_one(&db).await {
    //     Ok(row) => row,
    //     Err(err) => {
    //         eprintln!("Error inserting user data: {:?}", err);
    //         return Err(AppError::new(
    //             StatusCode::INTERNAL_SERVER_ERROR, 
    //             "Error inserting user data to the DB"
    //         ));
    //     },
    // };
    // println!("{:?}",row);
    
    // let user_db = UserDB {
    //     id: row.get(0),
    //     username: row.get::<String,_>("username"),
    //     password: row.get::<String,_>("password"),
    //     created_at: row.get::<DateTime<Utc>,_>("created_at"),
    //     deleted_at: row.get::<Option<DateTime<Utc>>,_>("deleted_at"),
    //     token: row.get::<Option<String>,_>("token"),
    // };
    // println!("{:?}",user_db);
}