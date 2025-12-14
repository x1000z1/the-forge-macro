// use std::sync::{Arc, Mutex, atomic::AtomicBool};
// pub struct Bot {
//     pub is_clicked: Arc<AtomicBool>,
//     pub is_luck: Arc<AtomicBool>,
//     pub is_sell: Arc<AtomicBool>,
//     pub potion_key: Arc<Mutex<String>>,
//     pub time_key: Arc<Mutex<u8>>,
// }
// impl Bot {
//     pub fn new() -> Self {
//         Self {
//             is_clicked: Arc::new(AtomicBool::new(false)),
//             is_luck: Arc::new(AtomicBool::new(false)),
//             is_sell: Arc::new(AtomicBool::new(false)),
//             potion_key: Arc::new(Mutex::new("3".to_string())),
//             time_key: Arc::new(Mutex::new(15)),
//         }
//     }
// }
