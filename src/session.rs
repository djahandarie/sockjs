use actix::*;

use protocol::Frame;
use context::SockJSContext;

/// Session state
#[derive(PartialEq, Debug)]
pub enum SessionState {
    /// Newly create session
    New,
    /// Transport is connected
    Running,
    /// Session interrupted
    Interrupted,
    /// Session is closed
    Closed,
}

#[derive(Debug)]
pub struct Message(pub String);

impl ResponseType for Message {
    type Item = ();
    type Error = ();
}

impl From<Message> for Frame {
    fn from(m: Message) -> Frame {
        Frame::Message(m.0)
    }
}

#[doc(hidden)]
pub enum SessionError {
    Acquired,
    Interrupted,
    Closing,
    InternalError,
}

/// This trait defines sockjs session
#[allow(unused_variables)]
pub trait Session: Actor<Context=SockJSContext<Self>> + Default + Handler<Message> {

    /// Method get called when transport acquires this session
    fn acquired(&mut self, ctx: &mut SockJSContext<Self>) {}

    /// Method get called when transport releases this session
    fn released(&mut self, ctx: &mut SockJSContext<Self>) {}
}
