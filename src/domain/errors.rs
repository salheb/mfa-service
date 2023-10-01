use std::{error::Error, fmt};

#[derive(Debug)]
pub struct AccountNotFoundError;

impl fmt::Display for AccountNotFoundError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "ERRTS001 - Account id not found.")
    }
}

impl Error for AccountNotFoundError {}

#[derive(Debug)]
pub struct SubAccountNotCreatedError;

impl fmt::Display for SubAccountNotCreatedError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "ERRTS002 - Sub Account can't be created. Please review payload.")
    }
}

impl Error for SubAccountNotCreatedError {}


#[derive(Debug)]
pub struct SubAccountNotFoundError;

impl fmt::Display for SubAccountNotFoundError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "ERRTS003 - Sub Account not found. Please review payload.")
    }
}

impl Error for SubAccountNotFoundError {}



#[derive(Debug)]
pub struct TokenNotGeneratedError;

impl fmt::Display for TokenNotGeneratedError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "ERRTS004 - MFA token could not be generated.")
    }
}

impl Error for TokenNotGeneratedError {}


#[derive(Debug)]
pub struct KafkaMessageCouldNotBeSentError;

impl fmt::Display for KafkaMessageCouldNotBeSentError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "ERRTS005 - Kafka message could not be sent.")
    }
}

impl Error for KafkaMessageCouldNotBeSentError {}


#[derive(Debug)]
pub struct TokenNotPersistedError;

impl fmt::Display for TokenNotPersistedError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "ERRTS006 - Token not persisted in database.")
    }
}

impl Error for TokenNotPersistedError {}