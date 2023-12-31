use chrono::DateTime;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;
use protobuf:: Message;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use ActionMessages::{ActionRequest, ActionResponse};

/// helper function for debugging variable types
fn print_type_of<T> (t: &T) {
    println!( "{}", std::any::type_name::<T>() );
}

// simplified function for thread sleeping
fn sleep_ms(time_ms:u64) {
    thread::sleep(std::time::Duration::from_millis(time_ms));
}

// function for generating current timestamp in string format
fn generate_timestamp()->String {
    let mut utc : DateTime<Utc> = Utc::now();    
    let timestamp_str = utc.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    timestamp_str
}

///////// Generic message type
#[derive(Debug)]
struct MessageEnvelope<T> {
    data: T,
    timestamp: String,
}

///////// Generic Publisher struct
struct Publisher<T> {
    messagesList: Arc<Mutex<Vec<MessageEnvelope<T>>>>,
}

impl<T: std::fmt::Debug> Publisher<T> {
    fn new(messagesList: Arc<Mutex<Vec<MessageEnvelope<T>>>>) -> Self {
        Publisher {
            messagesList
        }
    }

    fn publish(&self, data: T) {
        // Create and publish a message
        let mut messagesList = self.messagesList.lock().unwrap();
        // print_type_of(&messagesList);
        let timestamp = generate_timestamp();
        messagesList.push(MessageEnvelope { data, timestamp });
    }
}

///////// Generic Subscriber struct
struct Subscriber<T> {
    messagesList: Arc<Mutex<Vec<MessageEnvelope<T>>>>,
}

impl<T: Clone + std::fmt::Debug> Subscriber<T> {
    fn new(messagesList: Arc<Mutex<Vec<MessageEnvelope<T>>>>) -> Self {
        Subscriber { messagesList }
    }

    fn subscribe(&self) {
        let mut cnt:u32 = 0;
        loop {
            // Wait for new data
            let mut messagesList = self.messagesList.lock().unwrap();

            while !messagesList.is_empty() {
                // Retrieve and process the data
                let message = &messagesList[0];
                println!("Received: {:?}, timestamp: {}", message.data, message.timestamp );
                messagesList.remove(0);
            }

            drop(messagesList); // Release the lock
            sleep_ms(500);
            cnt = cnt+1;
            if cnt > 12 {
                break;
            }
        }
    }
}

///////////////////////////
// test and main functions
///////////////////////////
fn test_pubsub() {
    // shared data container for publishing and reading messages
    let messagesList = Arc::new(Mutex::new(Vec::new()));

    // Create instances of the Publisher and Subscriber with i32 data type
    let publisher : Publisher<i32>  = Publisher::new(Arc::clone(&messagesList));
    let subscriber : Subscriber<i32> = Subscriber::new(Arc::clone(&messagesList));

    // Spawn the subscriber in a separate thread
    let subscriber_handle = thread::spawn(move || {
        subscriber.subscribe();
    });

    // Publish some data in the main thread
    for i in 1..=10 {
        publisher.publish(i);
        println!("Published new message {}", i);
        sleep_ms(300);
    }

    // Wait for the subscriber thread to finish
    subscriber_handle.join().unwrap();
}

fn run_proto_pubsub() {
    // shared data container for publishing and reading messages
    let messagesList = Arc::new(Mutex::new(Vec::new()));

    // Create instances of the Publisher and Subscriber with i32 data type
    let publisher : Publisher<ActionRequest>  = Publisher::new(Arc::clone(&messagesList));
    let subscriber : Subscriber<ActionRequest> = Subscriber::new(Arc::clone(&messagesList));

    // Spawn the subscriber in a separate thread
    let subscriber_handle = thread::spawn(move || {
        subscriber.subscribe();
    });

    // Publish some data in the main thread
    for i in 1..=10 {
        let mut req_msg = ActionRequest::new();
        let id = Uuid::new_v4();
        req_msg.guid = id.to_string();
        req_msg.value = i;
        req_msg.details.push("aaaa".to_string());
        req_msg.details.push("zzzz".to_string());
        println!("Published new message {}", req_msg);
        
        publisher.publish(req_msg);
        sleep_ms(300);
    }

    // Wait for the subscriber thread to finish
    subscriber_handle.join().unwrap();
}

fn main() {
    run_proto_pubsub();
}