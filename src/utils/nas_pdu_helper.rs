use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};

// const PDU_SESSION_ESTABLISHMENT_ACCEPT__5GSM_CAUSE_IEI: u8 = 0x59;
const _PDU_SESSION_ESTABLISHMENT_ACCEPT_PDU_ADDRESS_IEI: u8 = 0x29;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_GPRS_TIMER_IEI: u8 = 0x56;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_SNSSAI_IEI: u8 = 0x22;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_ALWAYSON_PDU_SESSION_INDICATION_IEI: u8 = 0x80;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_MAPPED_EPS_BEARER_CONTEXTS_IEI: u8 = 0x75;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_EAP_MESSAGE_IEI: u8 = 0x78;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_QOS_FLOW_DESCRIPTIONS_IEI: u8 = 0x79;
// const PDU_SESSION_ESTABLISHMENT_ACCEPT_EPCO_IEI: u8 = 0x7B;
const _PDU_SESSION_ESTABLISHMENT_ACCEPT_DNN_IEI: u8 = 0x25;

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
// use std::mem::ManuallyDrop;

use std::alloc::alloc;
use std::alloc::Layout;
use std::slice;

use crate::utils::pdu::PduAddressType;
use crate::utils::tlv_ext_decoder::ExtProtoCfgOpts;
#[repr(C)]
#[derive(Debug)]
pub struct PduSessionEstablishmentAcceptMsg {
    // pub extendedprotocoldiscriminator: ExtendedProtocolDiscriminator,
    pub pdusessionidentity: PDUSessionIdentity,
    pub proceduretransactionidentity: ProcedureTransactionIdentity,
    // pub messagetype: MessageType,
    pub pdusessiontype: PDUSessionType,
    // pub sscmode: SSCMode,
    // qosrules: QOSRules,
    // sessionambr: SessionAMBR,
    // presence: u16,
    // _5gsmcause: _5GSMCause,
    pub pduaddress: PDUAddress,
    // gprstimer: GPRSTimer,
    // snssai: SNSSAI,
    // alwaysonpdusessionindication: AlwaysonPDUSessionIndication,
    // mappedepsbearercontexts: MappedEPSBearerContexts,
    // eapmessage: EAPMessage,
    // qosflowdescriptions: QOSFlowDescriptions,
    pub extendedprotocolconfigurationoptions: ExtProtoCfgOpts,
    pub dnn: DNN,
}

// pub type ExtendedProtocolDiscriminator = u8;

pub type PDUSessionIdentity = u8;

pub type ProcedureTransactionIdentity = u8;

// pub type MessageType = u8;

#[repr(C)]
#[derive(Debug, PartialEq)]

pub struct PDUSessionType {
    pub pdu_session_type_value: PduAddressType,
    pub spare: u8,
}

impl PDUSessionType {
    fn default() -> Self {
        PDUSessionType {
            pdu_session_type_value: PduAddressType::IPV4,
            spare: 0,
        }
    }
}

// #[repr(C)]
// #[derive(Debug)]

// pub struct SSCMode {
//     pub sscModeValue: u8,
//     pub spare: u8,
// }

#[repr(C)]
// pub struct PacketFilterContents {
//     pub component_type: u8,
//     pub component_value: OctetString,
// }

// // impl PacketFilterContents {
// //     fn default() -> Self {
// //         PacketFilterContents {
// //             component_type: 0,
// //             component_value: 0,
// //         }
// //     }
// // }

// #[repr(C)]
// pub struct Create_ModifyAndAdd_ModifyAndReplace {
//     pub packetfilteridentifier: u8,
//     pub packetfilterdirection: u8,
//     pub spare: u8,
//     pub lenghtofpacketfiltercontents: u8,
//     pub packetfiltercontents: PacketFilterContents,
// }

// impl Create_ModifyAndAdd_ModifyAndReplace {
//     fn default() -> Self {
//         Create_ModifyAndAdd_ModifyAndReplace {
//             packetfilteridentifier: 0,
//             packetfilterdirection: 0,
//             spare: 0,
//             lenghtofpacketfiltercontents: 0,
//             packetfiltercontents: 0,
//         }
//     }
// }

// #[repr(C)]
// pub struct QOSRulesIE {
//     pub qosruleidentifer: u8,
//     pub LengthofQoSrule: u16,
//     pub numberofpacketfilters: u8,
//     pub dqrbit: u8,
//     pub ruleoperationcode: u8,
//     pub packetfilterlist: PacketFilterList,
//     pub qosruleprecedence: u8,
//     pub qosflowidentifer: u8,
//     pub segregation: u8,
//     pub spare: u8,
// }
// impl QOSRulesIE {
//     fn default() -> Self {
//         QOSRulesIE {
//             qosruleidentifer: 0,
//             LengthofQoSrule: 0,
//             numberofpacketfilters: 0,
//             dqrbit: 0,
//             ruleoperationcode: 0,
//             packetfilterlist: 0,
//             qosruleprecedence: 0,
//             qosflowidentifer: 0,
//             segregation: 0,
//             spare: 0,
//         }
//     }
// }

// #[repr(C)]
// pub union PacketFilterList {
//     pub modifyanddelete: *mut ModifyAndDelete,
//     pub create_modifyandadd_modifyandreplace: *mut Create_ModifyAndAdd_ModifyAndReplace,
// }

// impl PacketFilterList {
//     fn default() -> Self {
//         PacketFilterList {
//             lengthofqosrulesie: std::ptr::null_mut(),
//             // qosrulesie: std::ptr::null_mut(),
//         }
//     }
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct QOSRules {
//     pub lengthofqosrulesie: u16,
//     pub qosrulesie: *mut QOSRulesIE,
// }
// // impl QOSRules {
// //     fn default() -> Self {
// //         QOSRules {
// //             lengthofqosrulesie: 0,
// //             qosrulesie: 0,
// //         }
// //     }
// // }

// #[repr(C)]
// #[derive(Debug)]

// pub struct SessionAMBR {
//     pub uint_for_session_ambr_for_downlink: u8,
//     pub session_ambr_for_downlink: u16,
//     pub uint_for_session_ambr_for_uplink: u8,
//     pub session_ambr_for_uplink: u16,
// }

// pub type _5GSMCause = u8;
#[repr(C)]
#[derive(Debug)]

pub struct PDUAddress {
    pub pdu_session_type_value: PduAddressType,
    // pub spare: u8,
    pub pdu_address_information: OctetString,
}
impl PDUAddress {
    pub fn default() -> Self {
        PDUAddress {
            pdu_session_type_value: PduAddressType::IPV4,
            // spare: 0,
            pdu_address_information: OctetString::default(),
        }
    }
}

fn decode_dnn(input: *mut u8) -> String {

    let len = unsafe { std::ptr::read(input.offset(0)) } as usize;
    
    let dnn_bytes = unsafe { 
      let ptr = input.offset(1) as *const u8;
      std::slice::from_raw_parts(ptr, len)
    };

    let dnn = std::str::from_utf8(dnn_bytes).unwrap();

    let dnn_components: Vec<&str> = dnn.split('.').collect();
    let dnn = dnn_components.join(".");

    // input = &mut input[len+1..];

    dnn
}



#[repr(C)]
#[derive(Debug)]



pub struct OctetString {
    pub length: u32,
    pub value: *mut u8,
}
impl OctetString {
    fn default() -> Self {
        OctetString {
            length: 0,
            value: std::ptr::null_mut(),
        }
    }
    pub fn set_value(&mut self, data: &[u8], start_index: usize, length: usize) {
        self.length = length as u32;
        self.value = std::ptr::null_mut(); // 重置value指针

        if length > 0 {
            let layout = Layout::array::<u8>(length).unwrap();
            self.value = unsafe { alloc(layout) as *mut u8 };
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr().add(start_index), self.value, length);
            }
        }
    }

    pub fn to_string(&mut self) -> &str {
        let string: &str;
        unsafe {
            let slice = slice::from_raw_parts(self.value, self.length.try_into().unwrap());
            string = std::str::from_utf8(slice).unwrap();
        };
        return string;
    }

    pub fn dnn_to_string(&mut self) -> String {
        let string  = decode_dnn(self.value);
        return string;
    }

    pub fn to_bytes_u8(&mut self) -> &[u8] {
        let mut _string: &str;
        let slice: &[u8];
        unsafe {
            slice = slice::from_raw_parts(self.value, self.length.try_into().unwrap());
        };
        return slice;
    }
}

// #[repr(C)]
// #[derive(Debug)]

// pub struct GPRSTimer {
//     pub timeValue: u8,
//     pub unit: u8,
// }

// #[repr(u8)]
// #[derive(Debug)]
// pub enum length_of_snssai_contents {
//     SST_LENGTH = 0b00000001,
//     SST_AND_MAPPEDHPLMNSST_LENGTH = 0b00000010,
//     SST_AND_SD_LENGTH = 0b00000100,
//     SST_AND_SD_AND_MAPPEDHPLMNSST_LENGTH = 0b00000101,
//     SST_AND_SD_AND_MAPPEDHPLMNSST_AND_MAPPEDHPLMNSD_LENGTH = 0b00001000,
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct SNSSAI {
//     pub len: length_of_snssai_contents,
//     pub sst: u8,
//     pub sd: [u8; 3],
//     pub mappedhplmnsst: u8,
//     pub mappedhplmnsd: [u8; 3],
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct AlwaysonPDUSessionIndication {
//     pub apsi_indication: u8,
//     pub spare: u8,
// }

// pub type MappedEPSBearerContexts = OctetString;
// pub type EAPMessage = OctetString;

// #[repr(C)]
// #[derive(Debug)]
// pub struct ParametersList {
//     pub parameteridentifier: u8,
//     pub lengthofparametercontents: u8,
//     pub parametercontents: ParametersListContents,
// }

// #[repr(C)]
// #[derive(Debug)]
// pub struct ParametersListContents {
//     pub _5qi: u8,
//     pub gfbrormfbr_uplinkordownlink: GFBROrMFBR_UpLinkOrDownLink,
//     pub averagingwindow: AveragingWindow,
//     pub epsbeareridentity: EpsBearerIdentity,
// }
// #[repr(C)]
// #[derive(Debug)]
// pub struct EpsBearerIdentity {
//     pub spare: u8,
//     pub identity: u8,
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct QOSFlowDescriptionsContents {
//     pub qfi: u8,
//     pub spare1: u8,
//     pub spare2: u8,
//     pub operationcode: u8,
//     pub numberofparameters: u8,
//     pub e: u8,
//     pub spare3: u8,
//     pub parameterslist: *mut ParametersList,
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct QOSFlowDescriptions {
//     pub qosflowdescriptionsnumber: u16,
//     pub qosflowdescriptionscontents: *mut QOSFlowDescriptionsContents,
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct GFBROrMFBR_UpLinkOrDownLink {
//     pub uint: u8,
//     pub value: u16,
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct AveragingWindow {
//     pub uplinkinmilliseconds: u8,
//     pub downlinkinmilliseconds: u8,
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct ExtendedProtocolConfigurationOptions {
//     pub configurationProtocol: u8,
//     pub spare: u8,
//     pub ext: u8,
//     pub numerofProtocolId: u8,
//     pub protocolId: *mut ProtocolIdContents,
// }

// #[repr(C)]
// #[derive(Debug)]

// pub struct ProtocolIdContents {
//     pub id: u16,
//     pub lengthofContents: u8,
//     pub contents: OctetString,
// }

pub type DNN = OctetString;

impl PduSessionEstablishmentAcceptMsg {
    pub fn new() -> Self {
        PduSessionEstablishmentAcceptMsg {
            // extendedprotocoldiscriminator: ExtendedProtocolDiscriminator::default(),
            pdusessionidentity: PDUSessionIdentity::default(),
            proceduretransactionidentity: ProcedureTransactionIdentity::default(),
            // messagetype: MessageType::default(),
            pdusessiontype: PDUSessionType::default(),
            // sscmode: SSCMode::default(),
            // qosrules: QOSRules::default(),
            // sessionambr: SessionAMBR::default(),
            // presence: 0,
            // _5gsmcause: _5GSMCause::default(),
            pduaddress: PDUAddress::default(),
            // gprstimer: GPRSTimer::default(),
            // snssai: SNSSAI::default(),
            // alwaysonpdusessionindication: AlwaysonPDUSessionIndication::default(),
            // mappedepsbearercontexts: MappedEPSBearerContexts::default(),
            // eapmessage: EAPMessage::default(),
            // qosflowdescriptions: QOSFlowDescriptions::default(),
            extendedprotocolconfigurationoptions: ExtProtoCfgOpts::default(),
            dnn: DNN::default(),
        }
    }

    pub fn get_dnn_name(&mut self) -> String {
        if self.dnn.length > 0 {
            return self.dnn.dnn_to_string();
        } else {
            return "".to_string();
        }
    }

    pub fn get_ipv4(&mut self) -> Result<IpAddr,&str> {
        let arr = self.pduaddress.pdu_address_information.to_bytes_u8();
        let mut _ipv6_str : String;
        let mut _ipv4_str : String;
        if arr.len() == 4 {
            let ipv4_bytes: [u8; 4] = [ arr[0], arr[1], arr[2],
                arr[3],
            ];
            let ipv4_addr = Ipv4Addr::from(ipv4_bytes);

            Ok(IpAddr::V4(ipv4_addr))
        } else if arr.len() == 8 {
             Err("")
        } else if arr.len() == 12 {
            
            let ipv6_bytes: [u8; 16] = [
                0, 0, 0, 0, 0, 0, 0, 0, arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6],
                arr[7],
            ];
            let ipv4_bytes: [u8; 4] = [ arr[8], arr[9], arr[10],
                arr[11],
            ];
            let _ipv6_addr = Ipv6Addr::from(ipv6_bytes);
            let ipv4_addr = Ipv4Addr::from(ipv4_bytes);
            Ok(IpAddr::V4(ipv4_addr))
        } else {
            Err("")
        }
    }

    pub fn _get_ipv6(&mut self) -> Result<IpAddr,&str> {
        let arr = self.pduaddress.pdu_address_information.to_bytes_u8();
        let mut _ipv6_str : String;
        let mut _ipv4_str : String;
        if arr.len() == 4 {
            Err("")

        } else if arr.len() == 8 {
            let ipv6_bytes: [u8; 16] = [ 0, 0, 0, 0, 0, 0, 0, 0, arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6],
            arr[7]
            ];
            let ipv6_addr = Ipv6Addr::from(ipv6_bytes);
            Ok(IpAddr::V6(ipv6_addr))

        } else if arr.len() == 12 {
            
            let ipv6_bytes: [u8; 16] = [
                0, 0, 0, 0, 0, 0, 0, 0, arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6],
                arr[7],
            ];
            let ipv4_bytes: [u8; 4] = [ arr[8], arr[9], arr[10],
                arr[11],
            ];
            let ipv6_addr = Ipv6Addr::from(ipv6_bytes);
            let _ipv4_addr = Ipv4Addr::from(ipv4_bytes);
            Ok(IpAddr::V6(ipv6_addr))
        } else {
            Err("")
        }
    }


}
