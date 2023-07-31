// #[derive(Debug, PartialEq)]
// pub(crate) enum SessionMessageType {
//     Unknown,
//     EstablishmentRequest,
//     EstablishmentAccept,
//     EstablishmentReject,
//     AuthenticationCommand,
//     AuthenticationComplete,
//     AuthenticationResult,
//     ModificationRequest,
//     ModificationReject,
//     ModificationCommand,
//     ModificationComplete,
//     ModificationCommandReject,
//     ReleaseRequest,
//     ReleaseReject,
//     ReleaseCommand,
//     ReleaseComplete,
// }
#[derive(Debug , PartialEq)]
pub enum PduAddressType {
    IPV4,
    IPV6,
    IPV4V6,
    Unknown
}

impl PduAddressType {
    pub fn from_u8(val: u8) -> PduAddressType {
        match val {
            0b00000001 => PduAddressType::IPV4,
            0b00000010 => PduAddressType::IPV6,
            0b00000011 => PduAddressType::IPV4V6,
            _ => PduAddressType::Unknown,
        }
    }
}

// impl SessionMessageType {
//     pub(crate) fn from_u8(val: u8) -> SessionMessageType {
//         match val {
//             0b00000001 => SessionMessageType::Unknown,
//             0b00000010 => SessionMessageType::EstablishmentRequest,
//             0b11000011 => SessionMessageType::EstablishmentAccept,
//             0b11000011 => SessionMessageType::EstablishmentReject,
//             0b11000101 => SessionMessageType::AuthenticationCommand,
//             0b11000110 => SessionMessageType::AuthenticationComplete,
//             0b11000111 => SessionMessageType::AuthenticationResult,
//             0b11001001 => SessionMessageType::ModificationRequest,
//             0b11001010 => SessionMessageType::ModificationReject,
//             0b11001011 => SessionMessageType::ModificationCommand,
//             0b11001100 => SessionMessageType::ModificationComplete,
//             0b11001101 => SessionMessageType::ModificationCommandReject,
//             0b11010001 => SessionMessageType::ReleaseRequest,
//             0b11010010 => SessionMessageType::ReleaseReject,
//             0b11010011 => SessionMessageType::ReleaseCommand,
//             0b11010100 => SessionMessageType::ReleaseComplete,
//             _ => SessionMessageType::Unknown,
//         }
//     }
// }

