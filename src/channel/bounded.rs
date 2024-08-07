use std::pin::Pin;

use futures::Stream;

/// inner module, used to group feature-specific imports
#[cfg(async_channel_impl = "tokio")]
mod inner {
    pub use tokio::sync::mpsc::error::{SendError, TryRecvError, TrySendError};

    use tokio::sync::mpsc::{Receiver as InnerReceiver, Sender as InnerSender};

    /// A receiver error returned from [`Receiver`]'s `recv`
    #[derive(Debug, PartialEq, Eq)]
    pub struct RecvError;

    impl std::fmt::Display for RecvError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, stringify!(RecvError))
        }
    }

    impl std::error::Error for RecvError {}

    /// A bounded sender, created with [`bounded`]
    pub struct Sender<T>(pub(super) InnerSender<T>);
    /// A bounded receiver, created with [`bounded`]
    pub struct Receiver<T>(pub(super) InnerReceiver<T>);
    /// A bounded stream, created with a channel
    pub struct BoundedStream<T>(pub(super) tokio_stream::wrappers::ReceiverStream<T>);

    /// Turn a `TryRecvError` into a `RecvError` if it's not `Empty`
    pub(super) fn try_recv_error_to_recv_error(e: TryRecvError) -> Option<RecvError> {
        match e {
            TryRecvError::Empty => None,
            TryRecvError::Disconnected => Some(RecvError),
        }
    }

    /// Create a bounded sender/receiver pair, limited to `len` messages at a time.
    #[must_use]
    pub fn bounded<T>(len: usize) -> (Sender<T>, Receiver<T>) {
        let (sender, receiver) = tokio::sync::mpsc::channel(len);
        (Sender(sender), Receiver(receiver))
    }
}

/// inner module, used to group feature-specific imports
#[cfg(async_channel_impl = "flume")]
mod inner {
    pub use flume::{RecvError, SendError, TryRecvError, TrySendError};

    use flume::{r#async::RecvStream, Receiver as InnerReceiver, Sender as InnerSender};

    /// A bounded sender, created with [`bounded`]
    pub struct Sender<T>(pub(super) InnerSender<T>);
    /// A bounded receiver, created with [`bounded`]
    pub struct Receiver<T>(pub(super) InnerReceiver<T>);
    /// A bounded stream, created with a channel
    pub struct BoundedStream<T: 'static>(pub(super) RecvStream<'static, T>);

    /// Turn a `TryRecvError` into a `RecvError` if it's not `Empty`
    pub(super) fn try_recv_error_to_recv_error(e: TryRecvError) -> Option<RecvError> {
        match e {
            TryRecvError::Empty => None,
            TryRecvError::Disconnected => Some(RecvError::Disconnected),
        }
    }

    /// Create a bounded sender/receiver pair, limited to `len` messages at a time.
    #[must_use]
    pub fn bounded<T>(len: usize) -> (Sender<T>, Receiver<T>) {
        let (sender, receiver) = flume::bounded(len);
        (Sender(sender), Receiver(receiver))
    }
}

/// inner module, used to group feature-specific imports
#[cfg(not(any(async_channel_impl = "flume", async_channel_impl = "tokio")))]
mod inner {
    pub use async_std::channel::{RecvError, SendError, TryRecvError, TrySendError};

    use async_std::channel::{Receiver as InnerReceiver, Sender as InnerSender};

    /// A bounded sender, created with [`channel`]
    pub struct Sender<T>(pub(super) InnerSender<T>);
    /// A bounded receiver, created with [`channel`]
    pub struct Receiver<T>(pub(super) InnerReceiver<T>);
    /// A bounded stream, created with a channel
    pub struct BoundedStream<T>(pub(super) InnerReceiver<T>);

    /// Turn a `TryRecvError` into a `RecvError` if it's not `Empty`
    pub(super) fn try_recv_error_to_recv_error(e: TryRecvError) -> Option<RecvError> {
        match e {
            TryRecvError::Empty => None,
            TryRecvError::Closed => Some(RecvError),
        }
    }

    /// Create a bounded sender/receiver pair, limited to `len` messages at a time.
    #[must_use]
    pub fn bounded<T>(len: usize) -> (Sender<T>, Receiver<T>) {
        let (sender, receiver) = async_std::channel::bounded(len);

        (Sender(sender), Receiver(receiver))
    }
}

pub use inner::*;

impl<T> Sender<T> {
    /// Send a value to the channel. May return a [`SendError`] if the receiver is dropped
    ///
    /// # Errors
    ///
    /// Will return an error if the receiver is dropped
    pub async fn send(&self, msg: T) -> Result<(), SendError<T>> {
        #[cfg(async_channel_impl = "flume")]
        let result = self.0.send_async(msg).await;
        #[cfg(not(all(async_channel_impl = "flume")))]
        let result = self.0.send(msg).await;

        result
    }

    /// Try to send a value over the channel. Will return immediately if the channel is full.
    ///
    /// # Errors
    /// - If the channel is full
    /// - If the channel is dropped
    pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
        self.0.try_send(msg)
    }
}

impl<T> Receiver<T> {
    /// Receive a value from te channel. This will async block until a value is received, or until a [`RecvError`] is encountered.
    ///
    /// # Errors
    ///
    /// Will return an error if the sender is dropped
    pub async fn recv(&mut self) -> Result<T, RecvError> {
        #[cfg(async_channel_impl = "flume")]
        let result = self.0.recv_async().await;
        #[cfg(async_channel_impl = "tokio")]
        let result = self.0.recv().await.ok_or(RecvError);
        #[cfg(not(any(async_channel_impl = "flume", async_channel_impl = "tokio")))]
        let result = self.0.recv().await;

        result
    }
    /// Turn this recever into a stream. This may fail on some implementations if multiple references of a receiver exist
    pub fn into_stream(self) -> BoundedStream<T>
    where
        T: 'static,
    {
        #[cfg(not(any(async_channel_impl = "flume", async_channel_impl = "tokio")))]
        let result = self.0;
        #[cfg(async_channel_impl = "tokio")]
        let result = tokio_stream::wrappers::ReceiverStream::new(self.0);
        #[cfg(async_channel_impl = "flume")]
        let result = self.0.into_stream();

        BoundedStream(result)
    }
    /// Try to receive a channel from the receiver. Will return immediately if there is no value available.
    ///
    /// # Errors
    ///
    /// Will return an error if the sender is dropped
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        self.0.try_recv()
    }
    /// Asynchronously wait for at least 1 value to show up, then will greedily try to receive values until this receiver would block. The resulting values are returned.
    ///
    /// It is guaranteed that the returning vec contains at least 1 value
    ///
    /// # Errors
    ///
    /// Will return an error if the sender is dropped
    pub async fn drain_at_least_one(&mut self) -> Result<Vec<T>, RecvError> {
        // Wait for the first message to come up
        let first = self.recv().await?;
        let mut ret = vec![first];
        loop {
            match self.try_recv() {
                Ok(x) => ret.push(x),
                Err(e) => {
                    if let Some(e) = try_recv_error_to_recv_error(e) {
                        tracing::error!(
                            "Tried to empty {:?} queue but it disconnected while we were emptying it ({} items are being dropped)",
                            std::any::type_name::<Self>(),
                            ret.len()
                        );
                        return Err(e);
                    }
                    break;
                }
            }
        }
        Ok(ret)
    }
    /// Drains the receiver from all messages in the queue, but will not poll for more messages
    ///
    /// # Errors
    ///
    /// Will return an error if the sender is dropped
    pub fn drain(&mut self) -> Result<Vec<T>, RecvError> {
        let mut result = Vec::new();
        loop {
            match self.try_recv() {
                Ok(t) => result.push(t),
                Err(e) => {
                    if let Some(e) = try_recv_error_to_recv_error(e) {
                        return Err(e);
                    }
                    break;
                }
            }
        }
        Ok(result)
    }
}

impl<T> Stream for BoundedStream<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        #[cfg(async_channel_impl = "flume")]
        return <flume::r#async::RecvStream<T>>::poll_next(Pin::new(&mut self.0), cx);
        #[cfg(async_channel_impl = "tokio")]
        return <tokio_stream::wrappers::ReceiverStream<T> as Stream>::poll_next(
            Pin::new(&mut self.0),
            cx,
        );
        #[cfg(not(any(async_channel_impl = "flume", async_channel_impl = "tokio")))]
        return <async_std::channel::Receiver<T> as Stream>::poll_next(Pin::new(&mut self.0), cx);
    }
}

// Clone impl
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

// Debug impl
impl<T> std::fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sender").finish()
    }
}
impl<T> std::fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Receiver").finish()
    }
}
