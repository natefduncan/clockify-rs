use std::collections::HashMap;
use std::fmt;

pub type EndpointParameters = HashMap<String, ParameterValue>;

#[derive(Debug, Clone)]
pub enum ParameterValue {
    String(String),
    Boolean(bool),
    Integer(u32)
}

impl fmt::Display for ParameterValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParameterValue::String(s) => {
                write!(f, "{}", s)
            }, 
            ParameterValue::Boolean(b) => {
                match b {
                    true => {
                        write!(f, "{}", "true")
                    },
                    _ => {
                        write!(f, "{}", "false")
                    }
                }
            }, 
            ParameterValue::Integer(i) => {
                write!(f, "{}", i.to_string())
            }
        }
    }
}

impl From<u32> for ParameterValue {
    fn from(i: u32) -> ParameterValue {
        ParameterValue::Integer(i)
    }
}

impl From<bool> for ParameterValue {
    fn from(b: bool) -> ParameterValue {
        ParameterValue::Boolean(b)
    }
}

impl From<String> for ParameterValue {
    fn from(s: String) -> ParameterValue {
        ParameterValue::String(s)
    }
}

#[derive(Debug, Clone)]
pub enum EndpointError {
    Unauthorized, // 401 
    Forbidden, // 403
    NotFound // 404
}

pub trait EndPoint {
    fn list(parameters: Option<EndpointParameters>) -> Result<Vec<Self>, EndpointError> where Self: Sized;
    fn get(id: u32, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized; 
    fn update(&self, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized;
    fn delete(id: u32, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized;
}
