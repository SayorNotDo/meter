pub enum FieldException {
    Require,
    NotAllowed,
    Mismatch,
    UnknownType,
}

impl ToString for FieldException {
    fn to_string(&self) -> String {
        let msg = match self {
            Self::Require => "require field/value missing",
            Self::NotAllowed => "field not allowed",
            Self::Mismatch => "mismatch value with type",
            Self::UnknownType => "unknown field type",
        };
        format!("Field Exception: {msg}")
    }
}

pub enum UserException {
    Forbidden,
}

impl ToString for UserException {
    fn to_string(&self) -> String {
        let msg = match self {
            Self::Forbidden => "access denied",
        };
        format!("User Exception: {msg}")
    }
}
