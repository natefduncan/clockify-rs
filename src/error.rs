#[derive(Debug)]
pub enum Error {
    Confy(confy::ConfyError), 
    Io(std::io::Error)
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
