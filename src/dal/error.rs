use crate::error::Error;
use diesel::r2d2::Error as DieselR2D2Error;
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;

impl From<DieselR2D2Error> for Error {
    fn from(value: DieselR2D2Error) -> Self {
        Error::from(value)
    }
}

impl From<DieselError> for Error {
    fn from(value: DieselError) -> Self {
        Error::from(value)
    }
}

impl From<R2D2Error> for Error {
    fn from(value: R2D2Error) -> Self {
        Error::from(value)
    }
}