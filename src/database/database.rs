#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    #[prost(message, repeated, tag = "1")]
    pub storage: ::prost::alloc::vec::Vec<database::StorageBox>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Database`.
pub mod database {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StorageBox {
        #[prost(message, repeated, tag = "1")]
        pub storage: ::prost::alloc::vec::Vec<StorageBox>,
        #[prost(message, repeated, tag = "2")]
        pub objects: ::prost::alloc::vec::Vec<storage_box::Object>,
    }
    /// Nested message and enum types in `StorageBox`.
    pub mod storage_box {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Object {
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
    }
}
