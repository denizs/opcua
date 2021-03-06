// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use service_types::impls::RequestHeader;
use service_types::BrowsePath;

/// Translates one or more paths in the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslateBrowsePathsToNodeIdsRequest {
    pub request_header: RequestHeader,
    pub browse_paths: Option<Vec<BrowsePath>>,
}

impl MessageInfo for TranslateBrowsePathsToNodeIdsRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::TranslateBrowsePathsToNodeIdsRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<TranslateBrowsePathsToNodeIdsRequest> for TranslateBrowsePathsToNodeIdsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.browse_paths);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.browse_paths)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let browse_paths: Option<Vec<BrowsePath>> = read_array(stream)?;
        Ok(TranslateBrowsePathsToNodeIdsRequest {
            request_header,
            browse_paths,
        })
    }
}
