use std::vec::Vec;
use std::collections::HashSet;
use primitive::*;
use encoded_query_buffer::EncodedQueryBuffer;
use query_result::QueryResult;

#[derive(Clone, Default, Debug)]
pub struct EncodedResultBuffer {
    pub data: HashSet<EncodedValue>,
}

impl EncodedResultBuffer {
    pub fn new() -> Self {
        EncodedResultBuffer::default()
    }

    // reposne format
    // query.id(8byte) + reuslt(0or1 1byte)
    pub fn build_query_response(
        &self,
        query_buffer: &EncodedQueryBuffer,
        response_vec: &mut Vec<u8>,
    ) {
        for query in query_buffer.queries.iter() {
            let mut result = QueryResult::new();
            result.query_id = query.id;
            for encoded_value in query.parameters.iter() {
                if self.data.contains(encoded_value) {
                    result.risk_level = 1;
                    break;
                };
            }
            response_vec.extend_from_slice(&result.to_be_bytes());
        }
    }
}