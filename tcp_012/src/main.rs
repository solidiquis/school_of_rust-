// One-thread per request TCP server with graceful exit... next time thread pool

use ctrlc;
use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Sender},
    time,
    thread::{self, JoinHandle},
};

enum Message {
    OngoingRequest(JoinHandle<()>),
    GracefulExit,
}

fn handle_connection(mut stream: TcpStream) -> JoinHandle<()> {
    thread::spawn(move || {
        // 1 KiB
        let mut buf: [u8; 1024] = [0; 1024];

        thread::sleep(time::Duration::from_secs(5));
        let bytes_read = stream.read(&mut buf).unwrap();

        let mut transformed_text = String::from_utf8_lossy(&buf).into_owned();
        transformed_text.make_ascii_uppercase();
        let output = transformed_text.trim_end_matches(char::from(0));

        let bytes_written = stream.write(output.as_bytes()).unwrap();

        println!("bytes read: {bytes_read}   bytes written: {bytes_written}");
    })
}

fn init_tcp_server(ln: TcpListener, tx: Sender<Message>) {
    thread::spawn(move || {
        for stream in ln.incoming() {
            let stream = stream.unwrap();
            let handle = handle_connection(stream);
            tx.send(Message::OngoingRequest(handle)).unwrap();
        }
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let ln = TcpListener::bind("localhost:3333")?;

    let (tx, rx) = mpsc::channel::<Message>();

    let tx_clone = tx.clone();
    ctrlc::set_handler(move || {
        tx_clone.send(Message::GracefulExit).unwrap();
    })?;

    init_tcp_server(ln, tx);

    let mut threads = Vec::new();

    while let Ok(message) = rx.recv() {
        match message {
            Message::OngoingRequest(handle) => threads.push(handle),
            Message::GracefulExit => break,
        }
    }

    let _ = threads
        .into_iter()
        .map(JoinHandle::join)
        .collect::<Vec<_>>();

    Ok(())
}
