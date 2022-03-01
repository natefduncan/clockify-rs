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
    NotFound, // 404
    Unknown(Option<u16>)
}

impl From<reqwest::Error> for EndpointError {
    fn from(error: reqwest::Error) -> EndpointError {
        match error.status() {
            Some(s) => {
                match s.as_u16() {
                    401 => EndpointError::Unauthorized, 
                    403 => EndpointError::Forbidden, 
                    404 => EndpointError::NotFound, 
                    _ => EndpointError::Unknown(Some(s.as_u16()))
                }
            }, 
            None => {
                EndpointError::Unknown(None)
            }
        }
    }
}

pub trait EndPoint {
    fn url() -> String;
    fn add_params(url: String, params: EndpointParameters) -> String {
        let mut output = String::new(); 
        for (key, value) in params.into_iter() {
            output = format!("&{}={}", key, value.to_string()); 
        }
        output
    }
    #[tokio::main]
    async fn list(&self, params: Option<EndpointParameters>) -> Result<Self, EndpointError>  
        where Self: Sized, for <'de> Self: serde::de::Deserialize<'de> {
        let mut url = String::new(); 
        if let Some(p) = params {
            url = Self::add_params(Self::url(), p); 
        } else {
            url = Self::url(); 
        }
        let struct_response = reqwest::get(url)
            .await?
            .json::<Self>()
            .await?;
        Ok(struct_response)
    }
    fn get(id: u32, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized; 
    fn update(&self, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized;
    fn delete(id: u32, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized;
}

