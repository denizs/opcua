// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryNextRequest {
    pub request_header: RequestHeader,
    pub release_continuation_point: Boolean,
    pub continuation_point: ByteString,
}

impl MessageInfo for QueryNextRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryNextRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryNextRequest> for QueryNextRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.release_continuation_point.byte_len();
        size += self.continuation_point.byte_len();
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.release_continuation_point.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let release_continuation_point = Boolean::decode(stream)?;
        let continuation_point = ByteString::decode(stream)?;
        Ok(QueryNextRequest {
            request_header: request_header,
            release_continuation_point: release_continuation_point,
            continuation_point: continuation_point,
        })
    }
}