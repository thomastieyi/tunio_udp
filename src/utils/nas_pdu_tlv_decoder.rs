use log::{debug, info};

use crate::{
    utils::pdu::PduAddressType,
    utils::nas_pdu_helper::{self, PduSessionEstablishmentAcceptMsg}, utils::tlv_ext_decoder::parse_extended_pco,
};

const PDU_SESSION_ESTABLISHMENT_ACCEPT_5_GSM_CAUSE_IEI: u8 = 0x59;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_RQ_TIMER_IEI: u8 = 0x56;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_ALWAYS_ON_IEI: u8 = 0x08;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_CP_ONLY_IEI: u8 = 0xc0;

// const PDU_SESSION_ESTABLISHMENT_ACCEPT_GPRS_TIMER_IEI: u8 = 0x56;

const PDU_SESSION_ESTABLISHMENT_ACCEPT_PDU_ADDRESS_IEI: u8 = 0x29;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_SNSSAI_IEI: u8 = 0x22;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_ALWAYSON_PDU_SESSION_INDICATION_IEI: u8 = 0x80;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_MAPPED_EPS_BEARER_CONTEXTS_IEI: u8 = 0x75;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_EAP_MESSAGE_IEI: u8 = 0x78;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_QOS_FLOW_DESCRIPTIONS_IEI: u8 = 0x79;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_EPCO_IEI: u8 = 0x7B;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_ATSSS_IEI: u8 = 0x77;
const PDU_SESSION_ESTABLISHMENT_ACCEPT_DNN_IEI: u8 = 0x25;

// const PDU_SESSION_ESTABLISHMENT_ACCEPT__5GSM_CAUSE_PRESENCE: u16 = 1 << 0;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_PDU_ADDRESS_PRESENCE: u16 = 1 << 1;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_GPRS_TIMER_PRESENCE: u16 = 1 << 2;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_SNSSAI_PRESENCE: u16 = 1 << 3;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_ALWAYSON_PDU_SESSION_INDICATION_PRESENCE: u16 = 1 << 4;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_MAPPED_EPS_BEARER_CONTEXTS_PRESENCE: u16 = 1 << 5;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_EAP_MESSAGE_PRESENCE: u16 = 1 << 6;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_QOS_FLOW_DESCRIPTIONS_PRESENCE: u16 = 1 << 7;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_EPCO_PRESENCE: u16 = 1 << 8;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_DNN_PRESENCE: u16 = 1 << 9;

pub const _NR_NETWORK_IF_MGMT_CREATE: u8 = 0x00;
pub const _NR_NETWORK_IF_MGMT_UPDATE: u8 = 0x01;
pub const _NR_NETWORK_IF_MGMT_DELETE: u8 = 0x11;

pub fn _tlv_decode_nr_network_if_mgm(data: &[u8]) -> Option<(u8, Vec<u8>)> {
    let mut index = 0;
    while index < data.len() {
        let current_tag = data[index];
        let length = data[index + 1] as usize;

        if current_tag == _NR_NETWORK_IF_MGMT_CREATE {
            let value = data[index + 2..index + 2 + length].to_vec();
            return Some((current_tag, value));
        }

        index += 2 + length;
    }

    None
}


/**
 * 3GPP TS 24501 8.3.2.1
 */
pub fn tlv_decode_pdu_session_establishment_accept(
    data: &[u8],
) -> Option<PduSessionEstablishmentAcceptMsg> {
    let mut index: usize = 0;
    let mut res: nas_pdu_helper::PduSessionEstablishmentAcceptMsg =
    nas_pdu_helper::PduSessionEstablishmentAcceptMsg::new();
    info!("{:#?}", data);
    //decode extended_protocol_discriminator
    index += 1;
    //decode_pdu_session_identity
    res.pdusessionidentity = data[index];
    index += 1;
    //decode_procedure_transaction_identity
    res.proceduretransactionidentity = data[index];
    index += 1;
    //decode_message_type
    index += 1;
    //seleted pdu session type and seleted ssc mode are in one octet!
    res.pdusessiontype.pdu_session_type_value = PduAddressType::from_u8(data[index] & 0b00000111);
    index += 1;
    //decode_qos_rules
    let value: u16 = (data[index] as u16) << 8 | data[index + 1] as u16;
    let length = value as usize;
    index += 2;
    index += length;
    //decode_session_ambr
    let length1 = data[index] as usize;
    index += 1;
    index += length1;

    //begin TLV

    while index < data.len() {
        let current_tag = data[index];
        let length: usize;
        let is_match = match current_tag {
            PDU_SESSION_ESTABLISHMENT_ACCEPT_MAPPED_EPS_BEARER_CONTEXTS_IEI
            | PDU_SESSION_ESTABLISHMENT_ACCEPT_EAP_MESSAGE_IEI
            | PDU_SESSION_ESTABLISHMENT_ACCEPT_QOS_FLOW_DESCRIPTIONS_IEI
            | PDU_SESSION_ESTABLISHMENT_ACCEPT_ATSSS_IEI
            | PDU_SESSION_ESTABLISHMENT_ACCEPT_EPCO_IEI => 1,
            PDU_SESSION_ESTABLISHMENT_ACCEPT_5_GSM_CAUSE_IEI
            | PDU_SESSION_ESTABLISHMENT_ACCEPT_RQ_TIMER_IEI => 2,
            PDU_SESSION_ESTABLISHMENT_ACCEPT_ALWAYSON_PDU_SESSION_INDICATION_IEI
            | PDU_SESSION_ESTABLISHMENT_ACCEPT_CP_ONLY_IEI => 3,
            _ => 0,
        };

        if is_match == 1 {
            let value: u16 = (data[index + 1] as u16) << 8 | data[index + 2] as u16;
            length = value as usize;
            debug!("TLV-E {} index {} len {}", current_tag, index, length);
        } else if is_match == 0 {
            length = data[index + 1] as usize;
            debug!("TLV {} index {}", current_tag, index);
        } else if is_match == 2 {
            length = 2;
            debug!("TV 2 {} index {}", current_tag, index);
        } else {
            length = 1;
            debug!("TV 1 {} index {}", current_tag, index);
        }

        if current_tag == PDU_SESSION_ESTABLISHMENT_ACCEPT_DNN_IEI {
            let value = data[index + 2..index + 2 + length].to_vec();
            let _dnn_index = res.dnn.set_value(&value, 0, length);
            // let string = std::str::from_utf8(&value).unwrap(); // 解析切片为字符串
            res.dnn.to_string();
        }

        if current_tag == PDU_SESSION_ESTABLISHMENT_ACCEPT_EPCO_IEI {
            let value = data[index ..index + 3 + length].to_vec();
            // let string = std::str::from_utf8(&value).unwrap(); // 解析切片为字符串
            // print!("{:#?}\n",value);
            res.extendedprotocolconfigurationoptions =  parse_extended_pco(&value).unwrap();
        }

        if current_tag == PDU_SESSION_ESTABLISHMENT_ACCEPT_PDU_ADDRESS_IEI {
            let ip_len;

            let value = data[index + 2..index + 2 + length].to_vec();
            res.pduaddress.pdu_session_type_value = PduAddressType::from_u8(value[0]);
            if res.pduaddress.pdu_session_type_value == PduAddressType::IPV4 {
                ip_len = 4;
            } else if res.pduaddress.pdu_session_type_value == PduAddressType::IPV6 {
                ip_len = 8;
            } else if res.pduaddress.pdu_session_type_value == PduAddressType::IPV4V6 {
                ip_len = 12;
            } else {
                ip_len = 4;
            }
            res.pduaddress
                .pdu_address_information
                .set_value(&value, 1, ip_len);
        }

        if is_match == 1 {
            index += 3 + length;
        } else if is_match == 0 {
            index += 2 + length;
        } else if is_match == 2 {
            index += length;
        } else {
            index += length;
        }
    }
    return Some(res);
}
