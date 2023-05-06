#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub quantity: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageBox {
    #[prost(message, repeated, tag = "1")]
    pub storage: ::prost::alloc::vec::Vec<StorageBox>,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Item>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    #[prost(message, repeated, tag = "1")]
    pub storage: ::prost::alloc::vec::Vec<StorageBox>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
