use std::{
    backtrace::Backtrace,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

#[macro_export]
macro_rules! raise {
    ($e:expr) => {
        $crate::Error::new_with_debug_info(
            $e,
            file!(),
            line!(),
            column!(),
            ::std::backtrace::Backtrace::force_capture(),
        )
    };
}

#[macro_export]
macro_rules! throw {
    ($e:expr) => {
        Err::<!, _>($crate::raise!($e))?
    };
}

#[derive(Debug)]
pub struct ErrorInfo {
    file: &'static str,
    line: u32,
    column: u32,
    backtrace: Backtrace,
}

impl ErrorInfo {
    pub fn file(&self) -> &'static str {
        self.file
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> u32 {
        self.column
    }

    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

#[derive(Debug)]
pub struct Error<T> {
    pub kind: Box<T>,
    pub info: Option<Box<ErrorInfo>>,
}

impl<T> DerefMut for Error<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl<T> Deref for Error<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl<T: StdError> StdError for Error<T> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.kind.source()
    }
}

impl<T: Display> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl<T> Error<T> {
    pub fn new_with_debug_info(
        kind: T,
        file: &'static str,
        line: u32,
        column: u32,
        backtrace: Backtrace,
    ) -> Self {
        Self {
            kind: Box::new(kind),
            info: Some(Box::new(ErrorInfo {
                file,
                line,
                column,
                backtrace,
            })),
        }
    }
}

pub type Result<T, E> = std::result::Result<T, Error<E>>;
