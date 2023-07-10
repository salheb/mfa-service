use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AccountNotFoundError;

impl fmt::Display for AccountNotFoundError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Account id not found.")
    }
}

impl Error for AccountNotFoundError {}

#[derive(Debug)]
pub struct SubAccountNotCreatedError;

impl fmt::Display for SubAccountNotCreatedError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Sub Account can't be created. Please review payload.")
    }
}

impl Error for SubAccountNotCreatedError {}


