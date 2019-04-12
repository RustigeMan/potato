use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

use sfml::window::Key;

#[derive(Debug)]
pub enum InpMsg {
    KeyDown(Key),
    KeyUp(Key),
}

pub fn run_thread(
    inp_recv: Receiver<InpMsg>,
    input_state: Arc<InputState>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Ok(message) = inp_recv.recv() {
            match message {
                InpMsg::KeyDown(key_code) => input_state.set_key_down(key_code),
                InpMsg::KeyUp(key_code) => input_state.set_key_up(key_code),
            }
        }
    })
}

pub struct InputState(Mutex<HashMap<Key, ()>>);
impl InputState {
    pub fn new() -> Arc<Self> {
        Arc::new(InputState(Mutex::new(HashMap::new())))
    }

    pub fn set_key_down(&self, key_code: Key) {
        self.0.lock().unwrap().insert(key_code, ());
    }
    pub fn set_key_up(&self, key_code: Key) {
        self.0.lock().unwrap().remove(&key_code).unwrap()
    }

    pub fn key_down(&self, key_code: Key) -> bool {
        match self.0.lock().unwrap().get(&key_code) {
            Some(()) => true,
            None => false,
        }
    }
}
