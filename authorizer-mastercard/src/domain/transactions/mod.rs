use crate::requests;
use aurora_8583::iso8583;

use std::convert::TryFrom;

#[derive(PartialEq, Debug)]
pub enum Transactions {
    OnlinePurchase(OnlinePurchaseTransaction),
    None,
}

#[derive(PartialEq, Debug)]
pub struct OnlinePurchaseTransaction(pub iso8583::ISOMessage);

impl TryFrom<iso8583::ISOMessage> for Transactions {
    type Error = iso8583::ISOMessageError;

    fn try_from(request: iso8583::ISOMessage) -> Result<Self, Self::Error> {
        match request {
            iso8583::ISOMessage {
                mti: iso8583::MessageTypeIndicator::AuthorizationRequest,
                pem: iso8583::POSEntryMode::EletronicCommerce,
                pcode: iso8583::PCode::Purchase,
                ..
            } => Ok(Transactions::OnlinePurchase(OnlinePurchaseTransaction(
                request,
            ))),
            _ => Err(iso8583::ISOMessageError::UnsupportedTransaction),
        }
    }
}

impl TryFrom<&requests::ISORequest> for Transactions {
    type Error = iso8583::ISOMessageError;

    fn try_from(value: &requests::ISORequest) -> Result<Self, Self::Error> {
        let iso = iso8583::ISOMessage::try_from(value)?;

        //TODO: Validar DE requeridos de acordo com TransactionType

        Transactions::try_from(iso)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_transaction_kind_should_be_invalid() {
        let fields = vec![
            requests::Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            requests::Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
            requests::Field {
                id: "3".to_string(),
                value: "000000".to_string(),
            },
            requests::Field {
                id: "22".to_string(),
                value: "90".to_string(),
            },
        ];

        let request = requests::ISORequest::new(fields);
        assert!(request.is_valid());

        let iso = iso8583::ISOMessage::try_from(&request);
        assert!(iso.is_ok());

        let iso = iso.unwrap();

        let transaction_kind = Transactions::try_from(iso);

        assert!(transaction_kind.is_err());

        assert_eq!(
            transaction_kind.unwrap_err(),
            iso8583::ISOMessageError::UnsupportedTransaction
        );
    }

    #[test]
    fn test_transaction_kind_should_be_online_purchase() {
        let fields = vec![
            requests::Field {
                id: "0".to_string(),
                value: "0100".to_string(),
            },
            requests::Field {
                id: "2".to_string(),
                value: "5276600404324025".to_string(),
            },
            requests::Field {
                id: "3".to_string(),
                value: "000000".to_string(),
            },
            requests::Field {
                id: "22".to_string(),
                value: "81".to_string(),
            },
        ];

        let request = requests::ISORequest::new(fields);
        assert!(request.is_valid());

        let iso = iso8583::ISOMessage::try_from(&request);
        assert!(iso.is_ok());

        let iso = iso.unwrap();

        let transaction_kind = Transactions::try_from(iso);

        assert!(transaction_kind.is_ok());
    }
}