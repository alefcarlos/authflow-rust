use std::fmt;

#[derive(PartialEq, Debug)]
pub enum MessageTypeIndicator {
    AuthorizationRequest,
    ReversalRequest,
}

impl fmt::Display for MessageTypeIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageTypeIndicator::AuthorizationRequest => write!(f, "AuthorizationRequest(0100)"),
            MessageTypeIndicator::ReversalRequest => write!(f, "ReversalRequest(0400"),
        }
    }
}

///An struct to represent ISO 8583-1
#[derive(Debug)]
pub struct ISOMessage {
    // DE 01
    pub mti: MessageTypeIndicator,
    pub card: Card,
    pub password : Password
}   

#[derive(Debug)]
pub struct Card {
    /// DE 02
    pub number: String,

    /// DE 23
    pub sequence: String,
}

#[derive(Debug)]
pub struct Password {
    /// DE 52
    pub value: String
}