// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use service_types::impls::RequestHeader;

#[derive(Debug, Clone, PartialEq)]
pub struct ModifySubscriptionRequest {
    pub request_header: RequestHeader,
    pub subscription_id: UInt32,
    pub requested_publishing_interval: Double,
    pub requested_lifetime_count: UInt32,
    pub requested_max_keep_alive_count: UInt32,
    pub max_notifications_per_publish: UInt32,
    pub priority: Byte,
}

impl MessageInfo for ModifySubscriptionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::ModifySubscriptionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ModifySubscriptionRequest> for ModifySubscriptionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.requested_publishing_interval.byte_len();
        size += self.requested_lifetime_count.byte_len();
        size += self.requested_max_keep_alive_count.byte_len();
        size += self.max_notifications_per_publish.byte_len();
        size += self.priority.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.requested_publishing_interval.encode(stream)?;
        size += self.requested_lifetime_count.encode(stream)?;
        size += self.requested_max_keep_alive_count.encode(stream)?;
        size += self.max_notifications_per_publish.encode(stream)?;
        size += self.priority.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let subscription_id = UInt32::decode(stream)?;
        let requested_publishing_interval = Double::decode(stream)?;
        let requested_lifetime_count = UInt32::decode(stream)?;
        let requested_max_keep_alive_count = UInt32::decode(stream)?;
        let max_notifications_per_publish = UInt32::decode(stream)?;
        let priority = Byte::decode(stream)?;
        Ok(ModifySubscriptionRequest {
            request_header,
            subscription_id,
            requested_publishing_interval,
            requested_lifetime_count,
            requested_max_keep_alive_count,
            max_notifications_per_publish,
            priority,
        })
    }
}
