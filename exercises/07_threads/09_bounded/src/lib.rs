// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        self.sender
            .send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| "Failed to send insert command".to_string())?;

        response_receiver
            .recv()
            .map_err(|_| "Failed to receive ticket ID".to_string())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        self.sender
            .send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| "Failed to send get command".to_string())?;

        response_receiver
            .recv()
            .map_err(|_| "Failed to receive ticket".to_string())
    }
}
use std::sync::mpsc::sync_channel;
pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(10);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                if response_channel.send(id).is_err() {
                    eprintln!("Failed to send ticket ID back to client");
                }
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                if response_channel.send(ticket.cloned()).is_err() {
                    eprintln!("Failed to send ticket back to client");
                }
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
