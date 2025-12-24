// TODO: Set `move_forward` to `true` in `ready` when you think you're done with this exercise.
//  Feel free to call an instructor to verify your solution!
// TODO: 当你认为你已经完成了这个练习时，在“ready”中将“move_forward”设置为“true”。
//  请随时致电讲师来验证您的解决方案！
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // If the thread is no longer running, this will panic
        // because the channel will be closed.
        // 如果线程不再运行，则会出现panic
        // 因为通道将被关闭。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // There's very little that we can check automatically in this exercise,
    // since our server doesn't expose any **read** actions.
    // We have no way to know if the inserts are actually happening and if they
    // are happening correctly.
    // 在本练习中我们可以自动检查的内容非常少，
    // 因为我们的服务器不公开任何**读取**操作。
    // 我们无法知道插入是否真的发生以及是否发生
    // 正在正确发生。
    let move_forward = true;

    assert!(move_forward);
}
