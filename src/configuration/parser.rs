use crate::configuration::Mode;

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Self::Test => String::from("test"),
            Self::Random => String::from("random"),
        }
    }
}
