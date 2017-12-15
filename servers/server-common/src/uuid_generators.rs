use std::thread;
use std::time::Instant;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};
use std::sync::Arc;
use uuid::{Uuid, UuidV1Context};
use byteorder::{NativeEndian, ByteOrder};

const UUID_V1_CONTEXT_COUNT_INIT: u16 = 42;

pub struct UuidV1Generator {
    ctx: Arc<UuidV1Context>,
    thread_id_hash: [u8; 8],
    creation: Instant,
}

impl UuidV1Generator {
    pub fn new(name: &String, context: Arc<UuidV1Context>) -> UuidV1Generator {
        let mut gen = UuidV1Generator {
            ctx: context,
            thread_id_hash: [0; 8],
            creation: Instant::now(),
        };

        let mut hasher = DefaultHasher::new();
        thread::current().id().hash(&mut hasher);
        name.hash(&mut hasher);
        NativeEndian::write_u64(&mut gen.thread_id_hash[..], hasher.finish());

        gen
    }

    pub fn new_context() -> UuidV1Context {
        UuidV1Context::new(UUID_V1_CONTEXT_COUNT_INIT)
    }
}

impl Iterator for UuidV1Generator {
    type Item = Uuid;

    fn next(&mut self) -> Option<Uuid> {
        let elapsed_time = Instant::now().duration_since(self.creation);
        let (secs, nanos) = (elapsed_time.as_secs(), elapsed_time.subsec_nanos());
        let node_id = &self.thread_id_hash[0..6];

        Uuid::new_v1(&self.ctx, secs, nanos, node_id).ok()
    }
}

lazy_static! {
    static ref GAME_NAMESPACE: Uuid = {
        Uuid::from_bytes(&[86, 176, 45, 56, 20, 122, 68, 206, 156, 87, 152, 132, 171, 175, 150, 140][..]).unwrap()
    };

    static ref PLAYER_NAMESPACE: Uuid = {
        Uuid::from_bytes(&[40, 147, 143, 180, 76, 248, 59, 91, 82, 154, 120, 2, 188, 28, 185, 188][..]).unwrap()
    };
}

pub fn generate_game_uuid(game_name: &str) -> Uuid {
    Uuid::new_v5(&*GAME_NAMESPACE, game_name)
}

pub fn generate_player_uuid(username: &str) -> Uuid {
    Uuid::new_v5(&*PLAYER_NAMESPACE, username)
}
