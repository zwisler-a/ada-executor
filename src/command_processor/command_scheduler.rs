use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::protocol::commands::AdaCommand;

pub fn listen(command_queue: Arc<Mutex<Vec<AdaCommand>>>) {
    let handle = thread::spawn(move || {
        loop {
            {
                let mut queue = command_queue.lock().unwrap();
                if queue.len() > 0 { log::info!("Working on {} queued commands", queue.len()); }
                while queue.len() > 0 {
                    let next_command = queue.pop();
                    if next_command.is_some() {
                        let command = next_command.unwrap();
                        log::debug!("Handle command {:?} with data [{:?}]", command.header, command.data)
                    }
                }
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });

    drop(handle);
}