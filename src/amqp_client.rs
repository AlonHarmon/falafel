use amiquip::{Result, QueueDeclareOptions, ConsumerOptions, Connection, ConsumerMessage};
use std::borrow::{Cow};


pub struct AmqpClient <'a, T> {
    action: &'a dyn Fn(Cow<str>) -> Result<T, ()>,
    connection: Connection,
    queue_name: &'a str
}

impl<T> AmqpClient<'_, T> {
    pub fn new<'a>(amqp_connection_address:&'a str, 
                    queue_name:&'a str, 
                    action:&'a dyn Fn(Cow<str>) -> Result<T, ()>) -> Result<AmqpClient<'a, T>> {

        let mut connection: Connection = amiquip::Connection::insecure_open(amqp_connection_address)?;

        let channel = connection.open_channel(None)?;
        let _ = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;

        log::debug!("Connected to {}", &amqp_connection_address);

        Ok(AmqpClient{
            action: action,
            connection: connection,
            queue_name: queue_name
        })
    }
}

impl<T> Iterator for AmqpClient<'_, T> {
    type Item = Result<T, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        let channel = self.connection.open_channel(None).ok()?;
        let queue = channel.queue_declare_passive(self.queue_name).ok()?;
        let consumer = queue.consume(ConsumerOptions::default()).ok()?;
        let message = consumer.receiver().iter().next()?;
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let output = Some((self.action)(body));
                consumer.ack(delivery).ok()?;
                output
            }
            other => {
                log::warn!("Consumer ended: {:?}", other);
                //TO DO: self.connection.close();
                Some(Err(()))
            }
        }
    }
}
