mod auth;
mod client;
mod common;
mod template_message;

pub use auth::*;
pub use client::*;
pub use common::*;
pub use template_message::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
