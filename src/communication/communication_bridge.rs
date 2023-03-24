use std::{fmt::Display, sync::{mpsc::{sync_channel, SyncSender, Receiver}}};

use bevy::prelude::info;




pub struct CommunicationBridge<T> {
    sender: SyncSender<T>,
    receiver: Receiver<T>,
}

unsafe impl<T> Sync for CommunicationBridge<T> { }


#[derive(Debug)]
pub enum ChannelError {
    ReceiveError,
    SendError
}

impl std::error::Error for ChannelError { }

type ChannelResult<T> = Result<T, ChannelError>;

impl Display for ChannelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelError::ReceiveError => write!(f, "Error while receiving message"),
            ChannelError::SendError => write!(f, "Error while sending message")
        }
    }
}

pub trait Channel where Self::MessageType : Send{
    type MessageType;

    fn send(&self, message: Self::MessageType) -> Result<(), ChannelError>;
    fn receive(&self) ->  ChannelResult<Self::MessageType>;
}

impl<T> CommunicationBridge<T> {
    pub fn new() -> Self {
        let (tx, rx) = sync_channel(100);
        Self {
            sender: tx,
            receiver: rx
        }
    }
}


impl<T> Channel for CommunicationBridge<T> where T : Send {
    type MessageType = T;

    fn send(&self, message: Self::MessageType) -> Result<(), ChannelError> {
        info!("Sending message");
        let res = self.sender.send(message).map_err(|_| ChannelError::SendError);
        res
    }

    fn receive(&self) -> ChannelResult<Self::MessageType> {
        self.receiver.try_recv().map_err(|_| ChannelError::ReceiveError)
    }
}