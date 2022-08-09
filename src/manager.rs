// use tokio::net::UdpSocket;

// use std::hash::Hash;
// // use crate::errors::*;
// use std::net::SocketAddr;

// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};

#[macro_export]
macro_rules! box_array {
    ($val:expr ; $len:expr) => {{
        // Use a generic function so that the pointer cast remains type-safe
        fn vec_to_boxed_array<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();

            let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; $len];

            unsafe { Box::from_raw(ptr) }
        }

        vec_to_boxed_array(vec![$val; $len])
    }};
}

pub const MAX_RECEIVE_BUFFER_SIZE: usize = 523944;
pub const MAX_SEND_BUFFER_SIZE: usize = 523944;

pub type ReceiveBuffer = Box<[u8; MAX_RECEIVE_BUFFER_SIZE]>;
pub type SendBuffer = Box<[u8; MAX_RECEIVE_BUFFER_SIZE]>;
pub type SocketID = usize;

pub struct Connection {
    pub seq: u32,
    pub ack: u32,

    pub previous_seq: u32,

    pub is_open: bool,
    pub last_response: u64,
}

// pub struct SocketsManager {

//     connections:Arc<Mutex<HashMap<SocketAddr,Connection>>>,

//     sockets:Arc<Mutex<HashMap<UdpSocket,Vec<SocketAddr>>>>

// }

// impl SocketsManager{
//     pub fn new() -> SocketsManager{
//         SocketsManager { connections: Arc::new(Mutex::new(HashMap::with_capacity(100))) }
//     }

//     pub fn new_socket(&self, socket:UdpSocket) -> bool{

//     }

// }
