use nutype::nutype;

use crate::UserFacingError;

#[nutype(validate(present, max_len = 50))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PollHeadline(String);

impl PollHeadline {
    pub const MAX_CHARS: usize = 50;
}

impl UserFacingError for PollHeadlineError {
    fn formatted_error(&self) -> &'static str {
        match self {
            PollHeadlineError::Missing => "Headline cannot be empty.",
            PollHeadlineError::TooLong => "Headline is too long. Must be at most 50 characters.",
        }
    }
}
#[nutype(validate(present, max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Headline(String);

impl Headline {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for HeadlineError {
    fn formatted_error(&self) -> &'static str {
        match self {
            HeadlineError::Missing => "Headline cannot be empty.",
            HeadlineError::TooLong => "Headline is too long. Must be at most 30 characters.",
        }
    }
}

#[nutype(validate(present, max_len = 100))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message(String);

impl Message {
    pub const MAX_CHARS: usize = 100;
}

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            MessageError::Missing => "Message cannot be empty.",
            MessageError::TooLong => "Message is too long. Must be at most 100 characters.",
        }
    }
}

#[nutype(validate(present, max_len = 60))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Caption(String);

impl Caption {
    pub const MAX_CHARS: usize = 60;
}

impl UserFacingError for CaptionError {
    fn formatted_error(&self) -> &'static str {
        match self {
            CaptionError::Missing => "Caption cannot be empty.",
            CaptionError::TooLong => "Caption is too long. Must be at most 60 characters.",
        }
    }
}

#[nutype(validate(present, max_len = 80))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PollChoiceDescription(String);

impl PollChoiceDescription {
    pub const MAX_CHARS: usize = 80;
}

impl UserFacingError for PollChoiceDescriptionError {
    fn formatted_error(&self) -> &'static str {
        match self {
            PollChoiceDescriptionError::Missing => "Poll choice cannot be empty.",
            PollChoiceDescriptionError::TooLong => {
                "Poll choice is too long. Must be at most 80 characters."
            }
        }
    }
}
