
const UNUSED_BITS: u16 = 1;
const EPOCH_BITS: u64 = 41;
const NODE_ID_BITS: u16 = 10;
const SEQUENCE_BITS: u16 = 12;

const MAX_NODE_ID: u64 = (2_i32.pow(NODE_ID_BITS as u32) - 1) as u64;
const MAX_SEQUENCE: u64 = (2_i32.pow(SEQUENCE_BITS as u32) - 1) as u64;

pub struct UniqueId {
    last_timestamp: u64,
    sequence: u64
}

pub struct UniqueIdBuilder {
    node_id: u16,
    unique_id: std::sync::Mutex<UniqueId>
}

impl UniqueIdBuilder {
    pub fn new(node_id: u16) -> Self {
        Self {
            node_id,
            unique_id: std::sync::Mutex::new(UniqueId::new())
        }
    }
}

impl UniqueId {
    fn new() -> Self {
        Self {
            last_timestamp: chrono::Utc::now().timestamp_millis() as u64,
            sequence: 0
        }
    }
}

pub struct UniqueIdGen(std::sync::Arc<UniqueIdBuilder>);

impl Clone for UniqueIdGen {
    fn clone(&self) -> Self {
        UniqueIdGen::new(self.0.node_id)
    }
}


impl UniqueIdGen {
    pub fn new(node_id: u16) -> Self {
        Self(std::sync::Arc::new(UniqueIdBuilder::new(node_id)))
    }

    pub fn next_id(&mut self) -> u64 {
        let mut unique_id = self.0.unique_id.lock().unwrap();
        let current_timestamp = chrono::Utc::now().timestamp_millis() as u64;
        if unique_id.last_timestamp < current_timestamp {
            unique_id.last_timestamp = current_timestamp;
            unique_id.sequence = 0
        } else {
            unique_id.sequence += 1;
            if unique_id.sequence == MAX_SEQUENCE {
                let mut latest_timestamp = chrono::Utc::now().timestamp_millis() as u64;
                while current_timestamp == latest_timestamp {
                    latest_timestamp = chrono::Utc::now().timestamp_millis() as u64;
                }
                unique_id.last_timestamp = latest_timestamp;
                unique_id.sequence = 0
            }
        }
        unique_id.last_timestamp << (NODE_ID_BITS + SEQUENCE_BITS) |
            (self.0.node_id as u64 )<< SEQUENCE_BITS |
            unique_id.sequence
    }
}

//
// fn get_binary(num: u64) {
//     if num == 0  {
//         return
//     }
//     get_binary(num/2);
//     print!("{}", num % 2);
// }
//
// pub fn next_sequence(node_id: u16) {
//
//     println!("{}", 1 << SEQUENCE_BITS);
//     println!("Max Node Id {} Max Seq {}", MAX_NODE_ID, MAX_SEQUENCE);
//
//     println!("Max Node ID binary: ");
//     get_binary(MAX_NODE_ID as u64);
//     println!();
//     let current_timestamp = chrono::Utc::now().timestamp_millis() as u64;
//     println!("current time stamp and binary {}", current_timestamp);
//     get_binary(current_timestamp);
//     //
//     let mut id = current_timestamp << (NODE_ID_BITS + SEQUENCE_BITS);
//     println!("\nId and binary {}", id);
//     get_binary(id);
//     println!();
//
//     println!("max node id with left shift {} {}", SEQUENCE_BITS,  MAX_NODE_ID << SEQUENCE_BITS);
//     get_binary((MAX_NODE_ID << SEQUENCE_BITS) as u64);
//     println!();
//     get_binary(id | (MAX_NODE_ID << SEQUENCE_BITS) as u64);
//     id = id | (MAX_NODE_ID << SEQUENCE_BITS) as u64;
//
//     println!();
//     get_binary(id | 1 as u64 );
//     println!("\n{}", id | 1 as u64);
//     get_binary(id | 2 as u64 );
//     println!("\n{}", id | 2 as u64);
// }