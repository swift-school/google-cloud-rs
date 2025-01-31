/// This resource represents a long-running operation that is the result of a
/// network API call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The server-assigned name, which is only unique within the same service that
    /// originally returns it. If you use the default HTTP mapping, the
    /// `name` should have the format of `operations/some/unique/name`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Service-specific metadata associated with the operation.  It typically
    /// contains progress information and common metadata such as create time.
    /// Some services might not provide such metadata.  Any method that returns a
    /// long-running operation should document the metadata type, if any.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<::prost_types::Any>,
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is
    /// available.
    #[prost(bool, tag="3")]
    pub done: bool,
    /// The operation result, which can be either an `error` or a valid `response`.
    /// If `done` == `false`, neither `error` nor `response` is set.
    /// If `done` == `true`, exactly one of `error` or `response` is set.
    #[prost(oneof="operation::Result", tags="4, 5")]
    pub result: ::core::option::Option<operation::Result>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// The operation result, which can be either an `error` or a valid `response`.
    /// If `done` == `false`, neither `error` nor `response` is set.
    /// If `done` == `true`, exactly one of `error` or `response` is set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// The error result of the operation in case of failure or cancellation.
        #[prost(message, tag="4")]
        Error(super::super::rpc::Status),
        /// The normal response of the operation in case of success.  If the original
        /// method returns no data on success, such as `Delete`, the response is
        /// `google.protobuf.Empty`.  If the original method is standard
        /// `Get`/`Create`/`Update`, the response should be the resource.  For other
        /// methods, the response should have the type `XxxResponse`, where `Xxx`
        /// is the original method name.  For example, if the original method name
        /// is `TakeSnapshot()`, the inferred response type is
        /// `TakeSnapshotResponse`.
        #[prost(message, tag="5")]
        Response(::prost_types::Any),
    }
}
/// The request message for [Operations.GetOperation][google.longrunning.Operations.GetOperation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationRequest {
    /// The name of the operation resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Operations.ListOperations][google.longrunning.Operations.ListOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsRequest {
    /// The name of the operation's parent resource.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// The standard list filter.
    #[prost(string, tag="1")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for [Operations.ListOperations][google.longrunning.Operations.ListOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsResponse {
    /// A list of operations that matches the specified filter in the request.
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
    /// The standard List next-page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Operations.CancelOperation][google.longrunning.Operations.CancelOperation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOperationRequest {
    /// The name of the operation resource to be cancelled.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Operations.DeleteOperation][google.longrunning.Operations.DeleteOperation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOperationRequest {
    /// The name of the operation resource to be deleted.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for [Operations.WaitOperation][google.longrunning.Operations.WaitOperation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitOperationRequest {
    /// The name of the operation resource to wait on.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum duration to wait before timing out. If left blank, the wait
    /// will be at most the time permitted by the underlying HTTP/RPC protocol.
    /// If RPC context deadline is also specified, the shorter one will be used.
    #[prost(message, optional, tag="2")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
}
/// A message representing the message types used by a long-running operation.
///
/// Example:
///
///   rpc LongRunningRecognize(LongRunningRecognizeRequest)
///       returns (google.longrunning.Operation) {
///     option (google.longrunning.operation_info) = {
///       response_type: "LongRunningRecognizeResponse"
///       metadata_type: "LongRunningRecognizeMetadata"
///     };
///   }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationInfo {
    /// Required. The message name of the primary return type for this
    /// long-running operation.
    /// This type will be used to deserialize the LRO's response.
    ///
    /// If the response is in a different package from the rpc, a fully-qualified
    /// message name must be used (e.g. `google.protobuf.Struct`).
    ///
    /// Note: Altering this value constitutes a breaking change.
    #[prost(string, tag="1")]
    pub response_type: ::prost::alloc::string::String,
    /// Required. The message name of the metadata type for this long-running
    /// operation.
    ///
    /// If the response is in a different package from the rpc, a fully-qualified
    /// message name must be used (e.g. `google.protobuf.Struct`).
    ///
    /// Note: Altering this value constitutes a breaking change.
    #[prost(string, tag="2")]
    pub metadata_type: ::prost::alloc::string::String,
}
# [doc = r" Generated client implementations."] pub mod operations_client { # ! [allow (unused_variables , dead_code , missing_docs)] use tonic :: codegen :: * ; # [doc = " Manages long-running operations with an API service."] # [doc = ""] # [doc = " When an API method normally takes long time to complete, it can be designed"] # [doc = " to return [Operation][google.longrunning.Operation] to the client, and the client can use this"] # [doc = " interface to receive the real response asynchronously by polling the"] # [doc = " operation resource, or pass the operation resource to another API (such as"] # [doc = " Google Cloud Pub/Sub API) to receive the response.  Any API service that"] # [doc = " returns long-running operations should implement the `Operations` interface"] # [doc = " so developers can have a consistent client experience."] pub struct OperationsClient < T > { inner : tonic :: client :: Grpc < T > , } impl < T > OperationsClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + HttpBody + Send + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as HttpBody > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor (inner : T , interceptor : impl Into < tonic :: Interceptor >) -> Self { let inner = tonic :: client :: Grpc :: with_interceptor (inner , interceptor) ; Self { inner } } # [doc = " Lists operations that match the specified filter in the request. If the"] # [doc = " server doesn't support this method, it returns `UNIMPLEMENTED`."] # [doc = ""] # [doc = " NOTE: the `name` binding allows API services to override the binding"] # [doc = " to use different resource name schemes, such as `users/*/operations`. To"] # [doc = " override the binding, API services can add a binding such as"] # [doc = " `\"/v1/{name=users/*}/operations\"` to their service configuration."] # [doc = " For backwards compatibility, the default name includes the operations"] # [doc = " collection id, however overriding users must ensure the name binding"] # [doc = " is the parent resource, without the operations collection id."] pub async fn list_operations (& mut self , request : impl tonic :: IntoRequest < super :: ListOperationsRequest > ,) -> Result < tonic :: Response < super :: ListOperationsResponse > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/google.longrunning.Operations/ListOperations") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Gets the latest state of a long-running operation.  Clients can use this"] # [doc = " method to poll the operation result at intervals as recommended by the API"] # [doc = " service."] pub async fn get_operation (& mut self , request : impl tonic :: IntoRequest < super :: GetOperationRequest > ,) -> Result < tonic :: Response < super :: Operation > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/google.longrunning.Operations/GetOperation") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Deletes a long-running operation. This method indicates that the client is"] # [doc = " no longer interested in the operation result. It does not cancel the"] # [doc = " operation. If the server doesn't support this method, it returns"] # [doc = " `google.rpc.Code.UNIMPLEMENTED`."] pub async fn delete_operation (& mut self , request : impl tonic :: IntoRequest < super :: DeleteOperationRequest > ,) -> Result < tonic :: Response < () > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/google.longrunning.Operations/DeleteOperation") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Starts asynchronous cancellation on a long-running operation.  The server"] # [doc = " makes a best effort to cancel the operation, but success is not"] # [doc = " guaranteed.  If the server doesn't support this method, it returns"] # [doc = " `google.rpc.Code.UNIMPLEMENTED`.  Clients can use"] # [doc = " [Operations.GetOperation][google.longrunning.Operations.GetOperation] or"] # [doc = " other methods to check whether the cancellation succeeded or whether the"] # [doc = " operation completed despite cancellation. On successful cancellation,"] # [doc = " the operation is not deleted; instead, it becomes an operation with"] # [doc = " an [Operation.error][google.longrunning.Operation.error] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,"] # [doc = " corresponding to `Code.CANCELLED`."] pub async fn cancel_operation (& mut self , request : impl tonic :: IntoRequest < super :: CancelOperationRequest > ,) -> Result < tonic :: Response < () > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/google.longrunning.Operations/CancelOperation") ; self . inner . unary (request . into_request () , path , codec) . await } # [doc = " Waits for the specified long-running operation until it is done or reaches"] # [doc = " at most a specified timeout, returning the latest state.  If the operation"] # [doc = " is already done, the latest state is immediately returned.  If the timeout"] # [doc = " specified is greater than the default HTTP/RPC timeout, the HTTP/RPC"] # [doc = " timeout is used.  If the server does not support this method, it returns"] # [doc = " `google.rpc.Code.UNIMPLEMENTED`."] # [doc = " Note that this method is on a best-effort basis.  It may return the latest"] # [doc = " state before the specified timeout (including immediately), meaning even an"] # [doc = " immediate response is no guarantee that the operation is done."] pub async fn wait_operation (& mut self , request : impl tonic :: IntoRequest < super :: WaitOperationRequest > ,) -> Result < tonic :: Response < super :: Operation > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/google.longrunning.Operations/WaitOperation") ; self . inner . unary (request . into_request () , path , codec) . await } } impl < T : Clone > Clone for OperationsClient < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } } impl < T > std :: fmt :: Debug for OperationsClient < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "OperationsClient {{ ... }}") } } }