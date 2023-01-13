// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    #[prost(message, optional, tag="1")]
    pub head_block: ::core::option::Option<BlockMetadata>,
    #[prost(uint64, tag="2")]
    pub total_block_count: u64,
    #[prost(uint64, tag="3")]
    pub total_transaction_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMetadata {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub number: u64,
    #[prost(string, tag="3")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub parent_number: u64,
    #[prost(string, tag="5")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub transaction_count: u64,
}
// @@protoc_insertion_point(module)
