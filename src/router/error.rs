use err_derive::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error(display = "a conflicting route already exists in the router")]
    RouteAlreadyExists,

    #[error(display = "handler execution failed")]
    HandlerExecutionFailed,
}
