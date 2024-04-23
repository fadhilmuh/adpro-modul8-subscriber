use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;
impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();

        // thread::sleep(ten_millis);

        println!(
            "In Fadhil's Computer [2206083464]. Messagereceived: {:?}",
            message
        );
        Ok(())
    }

    fn get_handler_action(&self) -> String {
        todo!()
    }
}

fn main() {
    let listener = CrosstownBus::new_queue_listener(
        "amqp://guest:guest@localhost:5672"
        .to_owned(),
    ).unwrap();

    _ = listener.listen(
        "user_created".to_owned(),
        UserCreatedHandler {},
        crosstown_bus::QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
        },
    );
    loop {}
}