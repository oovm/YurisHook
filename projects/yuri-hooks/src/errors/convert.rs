use super::*;

impl From<YuriErrorKind> for YuriError {
    fn from(value: YuriErrorKind) -> Self {
        Self { kind: Box::new(value) }
    }
}
