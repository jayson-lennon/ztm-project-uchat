use nutype::nutype;

#[nutype(validate(present, min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Username(String);

#[nutype(validate(present, min_len = 8))]
#[derive(AsRef, Clone, Serialize, Deserialize, PartialEq)]
pub struct Password(String);
