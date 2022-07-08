use std::borrow::Cow;
use falafel::amqp_client::AmqpClient;

/*fn p<'r>(aaa: Cow<str>) -> Result<&'r str, ()> {
    println!("{}", aaa);
    let tt = "test output";
    Ok(tt)
}*/


#[test]
fn test_amqp_client() {
    // menual test
    /*let consumer = AmqpClient::<&str>::new("amqp://guest:guest@localhost:5672", "receiver_queue", &p);
    for i in consumer.unwrap() {
        assert!(i.unwrap() == "test output");
    }*/
    assert!(true);
}
