use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
#[allow(irrefutable_let_patterns)]
pub fn server(receiver: Receiver<Command>) {
    loop {
        if receiver.recv().is_ok() {
            let mut store = store::TicketStore::new();
            let insert_command = receiver.recv().unwrap();

            if let Command::Insert(t) = insert_command {
                store.add_ticket(t);
            }
        }
    }
}
