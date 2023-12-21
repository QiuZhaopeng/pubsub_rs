use chrono::DateTime;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

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
	println!("{}", timestamp_str);
    timestamp_str
}

///////// Generic message type
#[derive(Debug)]
struct Message<T> {
    data: T,
    // todo: add timestamp
}

///////// Generic Publisher struct
struct Publisher<T> {
    messagesList: Arc<Mutex<Vec<Message<T>>>>,
}

impl<T: std::fmt::Debug> Publisher<T> {
    fn new(messagesList: Arc<Mutex<Vec<Message<T>>>>) -> Self {
        Publisher {
            messagesList
        }
    }

    fn publish(&self, data: T) {
        // Create and publish a message
        let mut messagesList = self.messagesList.lock().unwrap();
		// print_type_of(&messagesList);
        messagesList.push(Message { data });
		/*if messagesList.is_empty() {
		    println!("empty list");
	    }*/
    }
}

///////// Generic Subscriber struct
struct Subscriber<T> {
    messagesList: Arc<Mutex<Vec<Message<T>>>>,
}

impl<T: Clone + std::fmt::Debug> Subscriber<T> {
    fn new(messagesList: Arc<Mutex<Vec<Message<T>>>>) -> Self {
        Subscriber { messagesList }
    }

    fn subscribe(&self) {
		let mut cnt:u32 = 0;
        loop {
            // Wait for new data
            let mut messagesList = self.messagesList.lock().unwrap();

            while !messagesList.is_empty() {
                // Retrieve and process the data
                let message = &messagesList[0].data;
                println!("Received: {:?}", message);
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
// Main function
///////////////////////////
fn main() {
	// shared data container for publishing and reading messages
	let messagesList = Arc::new(Mutex::new(Vec::new()));

	generate_timestamp();
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
