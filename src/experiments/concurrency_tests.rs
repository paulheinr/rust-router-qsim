#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::mpsc::RecvError;
    use std::thread;
    use std::thread::JoinHandle;

    #[test]
    fn test_multiple_senders() {
        let (tx, rx) = mpsc::channel();
        let sender = tx.clone();

        thread::spawn(move || {
            let val = String::from("Thread 1");
            tx.send(val).unwrap();
        });

        thread::spawn(move || {
            let val = String::from("Thread 2");
            sender.send(val).unwrap();
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn test_multiple_senders_with_response() {
        let (query_sender1, query_receiver) = mpsc::channel();
        let (response_sender1, response_receiver1) = mpsc::channel();
        let (response_sender2, response_receiver2) = mpsc::channel();

        let query_sender2 = query_sender1.clone();

        let handle1 = thread::spawn(move || {
            let val = String::from("Thread 1");
            query_sender1.send((val, 1)).unwrap();
            let received_message = response_receiver1.recv().unwrap();
            println!("Thread 1 got '{received_message}'.")
        });

        let handle2 = thread::spawn(move || {
            let val = String::from("Thread 2");
            query_sender2.send((val, 2)).unwrap();
            let received_message = response_receiver2.recv().unwrap();
            println!("Thread 2 got '{received_message}'.")
        });

        let handle3 = thread::spawn(move || {
            let mut counter = 0;
            loop {
                match query_receiver.recv() {
                    Ok(x) => {
                        println!("Thread 3 received '{}' from Thread {}.", x.0, x.1);
                        println!("Thread 3 sends response.");
                        match x.1 {
                            1 => response_sender1.send("Hello Thread 1").unwrap(),
                            2 => response_sender2.send("Hello Thread 2").unwrap(),
                            _ => panic!("Got message from thread that should not exist.")
                        };
                        counter += 1;
                        if counter >= 2 {
                            break;
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }
}