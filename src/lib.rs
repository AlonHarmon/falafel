use amiquip::{Result, QueueDeclareOptions, ExchangeDeclareOptions, Publish, ConsumerOptions, ConsumerMessage, ExchangeType};
use std::borrow::{Cow};


pub trait ServicesWrappers {
    fn consume(&self, amqp_connection_address:&str, 
                queue_name:&str,
                action:&dyn Fn(Cow<str>)) -> Result<()>;

    fn publish(&self, amqp_connection_address:&str, 
                exchange_name:&str, 
                exchange_type:ExchangeType,
                ruting_key:&str,
                message: String) -> Result<()>;

    fn consume_and_publish(&self, amqp_connection_address:&str, 
                            queue_name:&str, 
                            exchange_name:&str, 
                            exchange_type:ExchangeType,
                            ruting_key:&str,
                            action:fn(Cow<str>) -> String) -> Result<()>;
}


pub struct AmqpClient ();


impl ServicesWrappers for AmqpClient {
    fn consume(&self, amqp_connection_address:&str, 
                queue_name:&str, 
                action:&dyn Fn(Cow<str>)) -> Result<()> {

        let mut connection = amiquip::Connection::insecure_open(amqp_connection_address)?;

        log::debug!("Connected to {}", &amqp_connection_address);

        let channel = connection.open_channel(None)?;
        let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;
        let consumer = queue.consume(ConsumerOptions::default())?;

        log::debug!("Starting to consume from queue {}", queue_name);

        for message in consumer.receiver().iter() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body);
                    action(body);
                    consumer.ack(delivery)?;
                }
                other => {
                    log::warn!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }

        connection.close()
    }

    fn publish(&self, amqp_connection_address:&str, 
                exchange_name:&str, 
                exchange_type:ExchangeType,
                ruting_key:&str,
                message: String) -> Result<()> {

        let mut connection = amiquip::Connection::insecure_open(amqp_connection_address)?;

        log::debug!("Connected to {}", &amqp_connection_address);

        let channel = connection.open_channel(None)?;
        let exchange = channel.exchange_declare(exchange_type, exchange_name, ExchangeDeclareOptions::default())?;
        
        let body = message.as_bytes();
        exchange.publish(Publish::new(body, ruting_key))?;
        
        log::debug!("Published to exchange {}", exchange_name);

        connection.close()
    }

    fn consume_and_publish(&self, amqp_connection_address:&str, 
                            queue_name:&str, 
                            exchange_name:&str, 
                            exchange_type:ExchangeType,
                            ruting_key:&str,
                            action:fn(Cow<str>) -> String) -> Result<()> {
        
        let action_wrapper:&dyn Fn(Cow<str>) = &|message| {
            let action_result = action(message);
            let exchange_type_clone = exchange_type.clone();
            self.publish(amqp_connection_address, exchange_name, exchange_type_clone, ruting_key, action_result)
            .unwrap();
        };
        self.consume(amqp_connection_address, queue_name, action_wrapper)?;
        Ok(())
    }
}
