use modules::EventEnvelope;
use tokio::sync::broadcast;

pub struct EventBus<T>
where
    T: Clone + Send + Sync,
{
    sender: broadcast::Sender<EventEnvelope<T>>,
}

impl<T> EventBus<T>
where
    T: Clone + Send + Sync,
{
    pub async fn publish(&self, event: EventEnvelope<T>) {
        let _ = self.sender.send(event);
    }

    pub async fn subscribe(&self) -> broadcast::Receiver<EventEnvelope<T>> {
        self.sender.subscribe()
    }
}
