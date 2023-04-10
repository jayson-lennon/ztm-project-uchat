#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPost {
    pub content: Content,
    pub options: NewPostOptions,
}
