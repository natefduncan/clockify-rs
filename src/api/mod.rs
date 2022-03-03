pub mod tag; 
pub mod project; 
pub mod common; 
pub mod task;
pub mod time_entry; 

use std::collections::HashMap;
use std::fmt;
use reqwest::blocking::{Client, RequestBuilder}; 

use serde::{Serialize, Deserialize}; 
use crate::clockify::{Config}; 

type EndpointParameters = HashMap<String, ParameterValue>;

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
    Unknown(String)
}

impl From<reqwest::Error> for EndpointError {
    fn from(error: reqwest::Error) -> EndpointError {
        match error.status() {
            Some(s) => {
                match s.as_u16() {
                    401 => EndpointError::Unauthorized, 
                    403 => EndpointError::Forbidden, 
                    404 => EndpointError::NotFound, 
                    _ => EndpointError::Unknown(format!("{:?}", error))
                }
            }, 
            None => {
                EndpointError::Unknown(format!("{:?}", error))
            }
        }
    }
}

pub trait EndPoint {

    fn endpoint(config: &Config) -> String;

    fn add_params(params: EndpointParameters) -> String {
        let mut output = String::new(); 
        for (key, value) in params.into_iter() {
            output = format!("&{}={}", key, value.to_string()); 
        }
        output
    }

    fn format_url(id: Option<&str>, params: Option<EndpointParameters>, config: &Config) -> String {
        let mut url = format!("{}{}", config.base_url, Self::endpoint(config)); 
        if let Some(i) = id {
            url = format!("{}/{}", url, i); 
        }
        if let Some(p) = params {
            url = format!("{}{}", url, Self::add_params(p)); 
        }
        url
    }

    fn set_api_key(request: RequestBuilder, config: &Config) -> RequestBuilder {
        request.header("X-API-KEY", config.api_key.as_ref().unwrap().clone())
    }

    fn list(client: &Client, config: &Config, params: Option<EndpointParameters>) -> Result<Vec<Self>, EndpointError>  
        where Self: Sized, for <'de> Self: serde::de::Deserialize<'de> {
        let url : String = Self::format_url(None, params, config); 
        let request : RequestBuilder = Self::set_api_key(client.get(url), config);
        let response = request
            .send()?
            .json::<Vec<Self>>()?; 
        Ok(response)
    }

    fn get(client: &Client, config: &Config, id: &str, params: Option<EndpointParameters>) -> Result<Self, EndpointError>
        where Self: Sized, for <'de> Self: serde::de::Deserialize<'de> {
        let url : String = Self::format_url(Some(id), params, config); 
        let request : RequestBuilder = Self::set_api_key(client.get(url), config);
        let response = request
            .send()?
            .json::<Self>()?; 
        Ok(response)

    }
    
    fn add(&self, client: &Client, config: &Config) -> Result<(), EndpointError> 
        where Self: Sized, for <'de> Self: serde::de::Deserialize<'de>, Self: Serialize {
        let url : String = Self::format_url(None, None, config);
        let request : RequestBuilder = Self::set_api_key(client.post(url), config);
        let response = request
            .json(self)
            .send()?
            .json::<Self>()?;
        Ok(())
    }
}

