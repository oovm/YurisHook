use super::*;

impl Error for YuriError {}

impl Debug for YuriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for YuriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for YuriErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YuriErrorKind::UnknownError => {
                write!(f, "UnknownError")
            }
            YuriErrorKind::GameNotFound => {
                write!(f, "GameNotFound")
            }
            YuriErrorKind::GameNotStart => {
                write!(f, "GameNotStart")
            }
            YuriErrorKind::SystemError { win32 } => {
                write!(f, "SystemError: {}", win32)
            }
        }
    }
}
