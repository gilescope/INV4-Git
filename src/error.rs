use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result},
};

pub struct ErrorWrap<T: Debug>(pub T);

impl<T: Debug> Debug for ErrorWrap<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Internal git-remote-gitarch error: {:?}", &self.0)
    }
}

impl<T: Debug> Display for ErrorWrap<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", &self)
    }
}

impl<T: Debug> Error for ErrorWrap<T> {}