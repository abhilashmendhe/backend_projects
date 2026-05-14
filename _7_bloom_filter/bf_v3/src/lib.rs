pub mod bf;
pub mod errors;

#[derive(Debug)]
pub enum BFCommand {
    INSERT(String),
    QUERY(String, tokio::sync::oneshot::Sender<(String, bool)>),
    SAVE(String),
}
