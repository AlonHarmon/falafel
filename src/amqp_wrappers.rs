use amiquip::{Result, ExchangeType, Publish, ExchangeDeclareOptions};
use std::borrow::{Cow};

use crate::amqp_client::AmqpClient;


pub trait AmqpServicesWrappers {
    fn consume(&self, amqp_connection_address:&str, 
        queue_name:&str,
        action: &dyn Fn(Cow<str>) -> Result<(), ()>) -> Result<()>;
    fn publish<'a>(&self, amqp_connection_address:&str, 
                exchange_name:&str, 
                exchange_type:ExchangeType,
                ruting_key:&str,
                messages: &mut dyn Iterator<Item = Result<&str, ()>>) -> Result<()>;

    fn consume_and_publish<'a>(&self, amqp_connection_address:&str, 
                            queue_name:&str, 
                            exchange_name:&str, 
                            exchange_type:ExchangeType,
                            ruting_key:&str,
                            action:&dyn Fn(Cow<str>) -> Result<&'a str, ()>) -> Result<()>;
}

struct AmqpWrappers ();

impl AmqpServicesWrappers for AmqpWrappers {
    fn consume(&self, amqp_connection_address:&str, 
        queue_name:&str,
        action: &dyn Fn(Cow<str>) -> Result<(), ()>) -> Result<()> {
            let consumer = AmqpClient::new(amqp_connection_address, queue_name, action)?;
            for _ in consumer {}
            Ok(())
        }

    fn publish<'a>(&self, amqp_connection_address:&str, 
                exchange_name:&str, 
                exchange_type:ExchangeType,
                ruting_key:&str,
                messages: &mut dyn Iterator<Item = Result<&str, ()>>) -> Result<()> {

        let mut connection = amiquip::Connection::insecure_open(amqp_connection_address)?;

        log::debug!("Connected to {}", &amqp_connection_address);

        let channel = connection.open_channel(None)?;
        let exchange = channel.exchange_declare(exchange_type, exchange_name, ExchangeDeclareOptions::default())?;
        
        for message in messages {
            let body = message.unwrap().as_bytes();
            exchange.publish(Publish::new(body, ruting_key))?;
            log::debug!("Published to exchange {}", exchange_name);
        }
        connection.close()
    }

    fn consume_and_publish<'a>(&self, amqp_connection_address:&str, 
                            queue_name:&str, 
                            exchange_name:&str, 
                            exchange_type:ExchangeType,
                            ruting_key:&str,
                            action:&dyn Fn(Cow<str>) -> Result<&'a str, ()>) -> Result<()> {
        
        let mut consumer = AmqpClient::<&str>::new(amqp_connection_address, queue_name, action)?;
        self.publish(amqp_connection_address, exchange_name, exchange_type, ruting_key, &mut consumer)?;
        Ok(())
    }
}
