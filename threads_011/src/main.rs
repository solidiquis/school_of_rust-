use std::{
    error::Error,
    sync::mpsc::channel,
    thread,
};
use walkdir::{WalkDir, DirEntry};

enum Message {
    NewEntry(DirEntry),
    Finished,
}

/// 1. Spin-up background thread that will receive messages from walkdir thread.
/// 2. Background thread will simply accumulate the size of all entries.
/// 3. Walkdir thread simply walks and sends entries to background thread.
fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel::<Message>();

    let handle = thread::spawn(move || {
        let mut total_size = 0;

        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Finished => break,
                Message::NewEntry(entry) => {
                    let Ok(md) = entry.metadata() else {
                        continue
                    };
                    total_size += md.len();
                }
            }
        }
        total_size
    });

    for entry in WalkDir::new(".") {
        let inner_entry = entry?;
        tx.send(Message::NewEntry(inner_entry))?;
    }
    tx.send(Message::Finished)?;

    let logical_disk_usage = handle.join().unwrap();

    println!("Total logical disk usage: {logical_disk_usage} B");

    Ok(())
}
