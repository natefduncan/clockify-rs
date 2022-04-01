#[derive(Debug)]
pub enum Error {
    Confy(confy::ConfyError), 
    Io(std::io::Error),
    MissingWorkspace,
    MissingTimeEntry,
    MissingProject,
    MissingUser,
    MissingData, 
    MissingApiKey, 
    Api(crate::api::EndpointError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::from(crate::api::EndpointError::from(e))
    }
}

impl From<crate::api::EndpointError> for Error {
    fn from(e: crate::api::EndpointError) -> Error {
        Error::Api(e)
    }
}

impl From<confy::ConfyError> for Error {
    fn from(e: confy::ConfyError) -> Error {
        Error::Confy(e)
    }
}   

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}   
