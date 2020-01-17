use err_derive::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error(display = "a conflicting route already exists in the router")]
    RouteAlreadyExists,
}

#[derive(Debug, Error)]
pub enum RouteHandlerError {
    #[error(display = "handler instantiation failed")]
    InstantiationError,

    #[error(display = "handler function not found")]
    FuncNotFound,

    #[error(display = "failed to serialize request to handler")]
    SerializationError,

    #[error(display = "handler execution failed")]
    ExecutionError,

    #[error(display = "failed to deserialize response from handler")]
    DeserializationError,
}
