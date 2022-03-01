use std::collections::HashMap;
use std::fmt;
use reqwest::blocking::RequestBuilder; 

use crate::clockify::{Clockify, BASE_URL}; 

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

    fn endpoint(clockify: &Clockify) -> String;

    fn add_params(params: EndpointParameters) -> String {
        let mut output = String::new(); 
        for (key, value) in params.into_iter() {
            output = format!("&{}={}", key, value.to_string()); 
        }
        output
    }

    fn format_url(params: Option<EndpointParameters>, clockify: &Clockify) -> String {
        let mut url = format!("{}{}", BASE_URL, Self::endpoint(clockify)); 
        if let Some(p) = params {
            url = format!("{}{}", url, Self::add_params(p)); 
        }
        url
    }

    fn set_api_key(request: RequestBuilder, clockify: &Clockify) -> RequestBuilder {
        request.header("X-API-KEY", clockify.api_key.clone())
    }

    fn list(params: Option<EndpointParameters>, clockify: &Clockify) -> Result<Self, EndpointError>  
        where Self: Sized, for <'de> Self: serde::de::Deserialize<'de> {
        let url : String = Self::format_url(params, clockify); 
        let request : RequestBuilder = Self::set_api_key(clockify.client.get(url), clockify);
        let response = request
            .send()?
            .json::<Self>()?; 
        Ok(response)
    }
    // fn get(id: u32, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized; 
    // fn update(&self, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized;
    // fn delete(id: u32, parameters: Option<EndpointParameters>) -> Result<Self, EndpointError> where Self: Sized;
}

