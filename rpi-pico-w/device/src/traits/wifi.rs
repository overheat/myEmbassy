use core::future::Future;
use embedded_nal_async::IpAddr;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Join<'a> {
    Open,
    Wpa { ssid: &'a str, password: &'a str },
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JoinError {
    Unknown,
    InvalidSsid,
    InvalidPassword,
    UnableToAssociate,
}

pub trait WifiSupplicant {
    type JoinFuture<'m>: Future<Output = Result<IpAddr, JoinError>>
    where
        Self: 'm;
    fn join<'m>(&'m mut self, join: Join<'m>) -> Self::JoinFuture<'m>;
}
