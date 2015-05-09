//! A bounded SPSC channel.

use arc::{Arc, ArcTrait};
use select::{Selectable, _Selectable};
use {Error, Sendable};
use endpoint;
use std::{ptr};
use std::raw::{TraitObject};

mod imp;
#[cfg(test)] mod test;
#[cfg(test)] mod bench;

/// Creates a new bounded SPSC channel.
///
/// ### Panic
///
/// Panics if `next_power_of_two(cap) * sizeof(T) >= isize::MAX`.
pub fn new<T: Sendable>(cap: usize) -> (Producer<T>, Consumer<T>) {
    let packet = Arc::new(imp::Packet::new(cap));
    packet.set_id(packet.unique_id());
    (Producer { data: packet.clone() }, Consumer { data: packet })
}

/// The producing half of a bounded SPSC channel.
pub struct Producer<T: Sendable> {
    data: Arc<imp::Packet<T>>,
}

impl<T: Sendable> Producer<T> {
    /// Sends a message over the channel. Blocks if the buffer is full.
    ///
    /// ### Errors
    ///
    /// - `Disconnected` - The receiver has disconnected.
    pub fn send_sync(&self, val: T) -> Result<(), (T, Error)> {
        self.data.send_sync(val)
    }

    /// Sends a message over the channel. Does not block if the buffer is full.
    ///
    /// ### Errors
    ///
    /// - `Full` - There is no space in the buffer.
    /// - `Disconnected` - The receiver has disconnected.
    pub fn send_async(&self, val: T) -> Result<(), (T, Error)> {
        self.data.send_async(val, false)
    }
}


impl<T: Sendable> Drop for Producer<T> {
    fn drop(&mut self) {
        self.data.disconnect_sender()
    }
}

unsafe impl<T: Sendable> Send for Producer<T> { }

/// The consuming half of a bounded SPSC channel.
pub struct Consumer<T: Sendable> {
    data: Arc<imp::Packet<T>>,
}

impl<T: Sendable> endpoint::Consumer<T> for Consumer<T> {
    fn recv_sync(&self) -> Result<T, Error> {
        self.data.recv_sync()
    }

    fn recv_async(&self) -> Result<T, Error> {
        self.data.recv_async(false)
    }
}


impl<T: Sendable> Drop for Consumer<T> {
    fn drop(&mut self) {
        self.data.disconnect_receiver()
    }
}

unsafe impl<T: Sendable> Send for Consumer<T> { }

impl<T: Sendable> Selectable for Consumer<T> {
    fn id(&self) -> usize {
        self.data.unique_id()
    }

    fn as_selectable(&self) -> ArcTrait<_Selectable> {
        unsafe { self.data.as_trait(ptr::read(&(&*self.data as &(_Selectable)) as *const _ as *const TraitObject)) }
    }
}
