#[derive(Debug)]
pub enum Error {
    ConnectionError(rumqttc::ConnectionError),
    ClientError(rumqttc::ClientError),
}

impl From<rumqttc::ClientError> for Error {
    fn from(v: rumqttc::ClientError) -> Self {
        Self::ClientError(v)
    }
}

impl From<rumqttc::ConnectionError> for Error {
    fn from(v: rumqttc::ConnectionError) -> Self {
        Self::ConnectionError(v)
    }
}
