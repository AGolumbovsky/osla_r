#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub word: String,
    pub part_of_speech: String,
    pub meaning: String,
    pub approved: bool,
}