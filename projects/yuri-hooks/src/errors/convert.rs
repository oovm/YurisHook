use super::*;
use win_memory::MemoryError;

impl From<YuriErrorKind> for YuriError {
    fn from(value: YuriErrorKind) -> Self {
        Self { kind: Box::new(value) }
    }
}

impl From<MemoryError> for YuriError {
    fn from(value: MemoryError) -> Self {
        match value {
            MemoryError::CreateSnapshotFailure => YuriErrorKind::UnknownError,
            MemoryError::IterateSnapshotFailure { win32 } => YuriErrorKind::SystemError { win32 },
            MemoryError::ProcessNotFound => YuriErrorKind::UnknownError,
            MemoryError::ModuleNotFound => YuriErrorKind::UnknownError,
            MemoryError::GetHandleError { win32 } => YuriErrorKind::SystemError { win32 },
            MemoryError::TerminateProcessError => YuriErrorKind::UnknownError,
            MemoryError::ReadMemoryError => YuriErrorKind::UnknownError,
            MemoryError::WriteMemoryError => YuriErrorKind::UnknownError,
            MemoryError::SignatureNotFound => YuriErrorKind::UnknownError,
            MemoryError::AddressOutOfBounds => YuriErrorKind::UnknownError,
            MemoryError::RIPRelativeFailed => YuriErrorKind::UnknownError,
            MemoryError::SystemError { win32 } => YuriErrorKind::SystemError { win32 },
        }
        .into()
    }
}
