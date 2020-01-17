use hyper::Method;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::io::Read;
use std::str;
use wasmer_runtime::memory::MemoryView;
use wasmer_runtime::{instantiate, Instance, Value};

use super::error::RouteHandlerError;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub enum RouteMethod {
    OPTIONS,
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    TRACE,
    CONNECT,
    PATCH,
}

impl RouteMethod {
    pub fn from_hyper(method: &Method) -> Self {
        match method {
            &Method::OPTIONS => Self::OPTIONS,
            &Method::GET => Self::GET,
            &Method::POST => Self::POST,
            &Method::PUT => Self::PUT,
            &Method::DELETE => Self::DELETE,
            &Method::HEAD => Self::HEAD,
            &Method::TRACE => Self::TRACE,
            &Method::CONNECT => Self::CONNECT,
            &Method::PATCH => Self::PATCH,
            _ => panic!(format!("unknown method: {:?}", method)),
        }
    }
}

// static WASM: &'static [u8] = include_bytes!(
//     "../../examples/yell-api-rust/target/wasm32-unknown-unknown/release/yell_api.wasm"
// );
// static WASM: &'static [u8] = include_bytes!("../../examples/yell-api-c/main.wasm");

pub struct MemoryViewReader<'a> {
    view: MemoryView<'a, u8>,
    ptr: usize,
}

impl<'a> Read for MemoryViewReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        buf[0] = Cell::get(&self.view[self.ptr]);
        self.ptr += 1;
        Ok(1)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RouteHandler {
    #[serde(with = "crate::utils::serde_base64")]
    pub code: Vec<u8>,
}

impl RouteHandler {
    pub fn run<'a, I, O>(&self, input: &I) -> Result<O, RouteHandlerError>
    where
        I: Serialize + ?Sized,
        O: DeserializeOwned,
    {
        let serialized_input = RouteHandler::serialize_input(input)?;
        let mut instance = RouteHandler::instantiate(&self.code)?;
        let output_reader = RouteHandler::execute(&mut instance, serialized_input)?;
        let output = Self::deserialize_output(output_reader)?;
        Ok(output)
    }

    fn serialize_input<I>(input: &I) -> Result<Vec<u8>, RouteHandlerError>
    where
        I: Serialize + ?Sized,
    {
        match rmp_serde::encode::to_vec(input) {
            Ok(serialized) => Ok(serialized),
            Err(_error) => Err(RouteHandlerError::SerializationError),
        }
    }

    fn instantiate(code: &Vec<u8>) -> Result<Instance, RouteHandlerError> {
        let state = wasmer_wasi::state::WasiState::new("handler")
            .build()
            .unwrap();

        let import_object = wasmer_wasi::generate_import_object_from_state(
            state,
            wasmer_wasi::WasiVersion::Snapshot1,
        );

        match instantiate(code.as_slice(), &import_object) {
            Ok(instance) => Ok(instance),
            Err(error) => {
                println!("error: {:?}", error);
                Err(RouteHandlerError::InstantiationError)
            }
        }
    }

    fn execute<'a>(
        instance: &'a mut Instance,
        input: Vec<u8>,
    ) -> Result<MemoryViewReader<'a>, RouteHandlerError> {
        let memory = instance.context_mut().memory(0);

        // write the input string into the lineary memory
        for (byte, cell) in input
            .iter()
            .zip(memory.view()[0 as usize..(input.len()) as usize].iter())
        {
            cell.set(*byte);
        }

        match instance.dyn_func("handler") {
            Ok(func) => match func.call(&[Value::I32(0)]) {
                Ok(values) => match values.as_slice() {
                    [Value::I32(output_ptr)] => Ok(MemoryViewReader {
                        view: instance.context().memory(0).view(),
                        ptr: *output_ptr as usize,
                    }),
                    _ => Err(RouteHandlerError::DeserializationError),
                },
                Err(error) => {
                    println!("error: {:?}", error);
                    Err(RouteHandlerError::ExecutionError)
                }
            },
            Err(error) => {
                println!("error: {:?}", error);
                Err(RouteHandlerError::FuncNotFound)
            }
        }
    }

    fn deserialize_output<O>(reader: MemoryViewReader) -> Result<O, RouteHandlerError>
    where
        O: DeserializeOwned,
    {
        match rmp_serde::decode::from_read(reader) {
            Ok(deserialized) => Ok(deserialized),
            Err(error) => {
                println!("error: {:?}", error);
                Err(RouteHandlerError::DeserializationError)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub method: RouteMethod,
    pub handler: RouteHandler,
}

impl Route {
    pub fn conflicts(&self, other: &Route) -> bool {
        self.path == other.path && self.method == other.method
    }
}
