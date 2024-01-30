use std::sync::{mpsc, Arc, RwLock};

#[derive(Debug)]
enum Message<T> {
    Get { key: String },
    Set { key: String, value: T },
}

type MyType = String;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send(Message::Set {
            key: "foo".to_string(),
            value: "bar".to_string(),
        })
        .unwrap();
    });

    tokio::spawn(async move {
        tx2.send(Message::Get {
            key: "foo".to_string(),
        })
        .unwrap();
    });
}
