use super::types::*;
use core::future::Future;

/// API for accessing LoRa modules
pub trait LoraDriver {
    type JoinFuture<'a>: Future<Output = Result<(), LoraError>>
    where
        Self: 'a;
    /// Join a LoRaWAN network with the given connect mode.
    fn join<'a>(&'a mut self, mode: JoinMode) -> Self::JoinFuture<'a>;

    type SendFuture<'a>: Future<Output = Result<(), LoraError>>
    where
        Self: 'a;
    /// Send data on a specific port with a given quality of service.
    fn send<'a>(&'a mut self, qos: QoS, port: Port, data: &'a [u8]) -> Self::SendFuture<'a>;

    type SendRecvFuture<'a>: Future<Output = Result<usize, LoraError>>
    where
        Self: 'a;
    /// Send data on a specific port with a given quality of service. If the LoRa module receives
    /// any data as part of the confirmation, write it into the provided buffer and return the size of the data read.
    fn send_recv<'a>(
        &'a mut self,
        qos: QoS,
        port: Port,
        data: &'a [u8],
        rx: &'a mut [u8],
    ) -> Self::SendRecvFuture<'a>;
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LoraError {
    JoinError,
    AckTimeout,
    NotReady,
    SendError,
    RecvError,
    RecvTimeout,
    RecvBufferTooSmall,
    NotInitialized,
    NotImplemented,
    UnsupportedRegion,
    OtherError,
}
