use crate::data::TicketDraft;
use crate::store::TicketStore;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
// 通过生成服务器线程来启动系统。
// 它返回一个可以使用的 `Sender` 实例
// 由一个或多个客户端与服务器交互
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
// TODO：服务器任务应该**永不**停止。
//  进入循环：等待命令出现
//  通道，然后执行它，然后开始等待
//  用于下一个命令。
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Insert(ticket_draft) => {
                store.add_ticket(ticket_draft);
            }
        }
    }
}
