use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::network::data_container::Container;

pub struct Node {
    pub(crate) id: Uuid,
    execute_fn: Box<dyn Fn(Container) + Send + Sync>,
    dependents: HashMap<Uuid, Arc<Mutex<Node>>>,
}

impl Node {
    pub fn new<F>(id: Uuid, execute_fn: F) -> Self
        where
            F: Fn(Container) + 'static + Send + Sync,
    {
        Node {
            id,
            execute_fn: Box::new(execute_fn),
            dependents: HashMap::new(),
        }
    }

    pub(crate) fn add_dependent(&mut self, dependent: Arc<Mutex<Node>>) {
        self.dependents.insert(dependent.lock().unwrap().id, dependent.clone());
    }

    pub fn execute(&self, arguments: Container) {
        log::debug!("Arguments: {:?}", arguments);
        (self.execute_fn)(arguments);
    }

    pub fn propagate(&self, arguments: Container) {
        let cloned_arguments = arguments.clone();
        for dependent in &self.dependents {
            let node = dependent.1.lock().unwrap();
            log::debug!("Propagate call to {}({:?})", node.id, arguments);
            (node.execute_fn)(cloned_arguments.clone());
        }
    }
}