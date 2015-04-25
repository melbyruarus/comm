use {Error, Sendable};
use select::Selectable;

pub trait Consumer<'a, T: Sendable+'a> : Sized + Selectable<'a> {
    /// Receives a message over this channel. Blocks until a message is available.
    ///
    /// ### Errors
    ///
    /// - `Disconnected` - No message is available and the sender has disconnected.
    fn recv_sync(&self) -> Result<T, Error>;

    /// Receives a message over this channel. Does not block if no message is available.
    ///
    /// ### Errors
    ///
    /// - `Disconnected` - No message is available and the sender has disconnected.
    /// - `Empty` - No message is available.
    fn recv_async(&self) -> Result<T, Error>;
}
