// TODO: you have something to do in each of the modules in this crate!
// TODO：此 crate 中的每个模块都有一些任务需要完成！
mod description;
mod status;
mod title;

// A common pattern in Rust is to split code into multiple (private) modules
// and then re-export the public parts of those modules at the root of the crate.
//
// This hides the internal structure of the crate from your users, while still
// allowing you to organize your code however you like.
//
// Rust 中常见的模式是将代码拆分成多个（私有）模块
// 然后将这些模块的公共部分重新导出到 crate 的根目录。
//
// 这样可以对用户隐藏 crate 的内部结构，同时仍然
// 允许您以任何您喜欢的方式组织代码。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// We no longer need to make the fields private!
// Since each field encapsulates its own validation logic, there is no risk of
// a user of `Ticket` modifying the fields in a way that would break the
// invariants of the struct.
//
// Careful though: if you had any invariants that spanned multiple fields, you
// would need to ensure that those invariants are still maintained and go back
// to making the fields private.
//
// 我们不再需要将字段设为私有！
// 由于每个字段都封装了自己的验证逻辑，因此不存在以下风险：
// `Ticket` 的用户不会以任何方式修改字段，从而破坏
// 结构体的不变式。
//
// 但请注意：如果您有任何跨越多个字段的不变式，则
// 您需要确保这些不变式仍然有效，并重新
// 将字段设为私有。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
