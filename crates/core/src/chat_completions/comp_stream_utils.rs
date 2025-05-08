use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};


pub struct ChunkStream<S>
where
    S: Stream<Item = reqwest::Result<bytes::Bytes>> + Send + Sync + 'static,
{
    inner: Pin<Box<S>>,
}

impl<S> ChunkStream<S>
where
    S: Stream<Item = reqwest::Result<bytes::Bytes>> + Send + Sync + 'static,
{
    pub(crate) fn new(stream: S) -> Self {
        ChunkStream {
            inner: Box::pin(stream),
        }
    }
}

impl<S> Stream for ChunkStream<S>
where
    S: Stream<Item = reqwest::Result<bytes::Bytes>> + Send + Sync + 'static,
{
    type Item = reqwest::Result<bytes::Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}
