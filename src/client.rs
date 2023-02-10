use anyhow::Result;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::mpsc::{Receiver, Sender},
};
use crate::{ClientEvent, ServerEvent};

pub struct Client<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    stream: S,
    width: u16,
    height: u16,
}

impl<S> Client<S>
where S: AsyncRead + AsyncWrite + Unpin
{
    pub fn new(stream: S, width: u16, height: u16) -> Self {
        Self { stream, width, height }
    }

    pub async fn run(&mut self, 
        client_tx: Sender<ServerEvent>,
        mut canvas_rx: Receiver<ClientEvent>,
    ) -> Result<()> {        
        let message = ClientEvent::FramebufferUpdateRequest {
            incremental: false, x: 0, y: 0,
            width: self.width, height: self.height,
        };
        message.write(&mut self.stream).await?;

        loop {
            tokio::select! {
                server_msg = ServerEvent::read(&mut self.stream) => {
                    let message = server_msg?;
                    client_tx.send(message).await?
                }
                client_msg = canvas_rx.recv() => {
                    if let Some(client_msg) = client_msg {
                        client_msg.write(&mut self.stream).await?;
                    }
                }
            }
        }
    }
}

