macro_rules! new_id {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            serde::Deserialize,
            serde::Serialize,
            PartialEq,
            Ord,
            PartialOrd,
        )]
        #[cfg_attr(feature = "query", derive(DieselNewType))]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn into_inner(self) -> uuid::Uuid {
                self.0
            }

            pub fn as_uuid(&self) -> &uuid::Uuid {
                &self.0
            }

            pub fn to_string(&self) -> String {
                self.0.to_string()
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl From<uuid::Uuid> for $name {
            fn from(id: uuid::Uuid) -> Self {
                $name(id)
            }
        }

        impl std::str::FromStr for $name {
            type Err = IdError;
            fn from_str(id: &str) -> Result<Self, Self::Err> {
                uuid::Uuid::try_parse(id)
                    .map(|id| id.into())
                    .map_err(|_| IdError::Parse)
            }
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("failed to parse ID")]
    Parse,
}

new_id!(UserId);
new_id!(SessionId);
