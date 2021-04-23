pub mod dungeons {
    mod gen;
    pub mod inst;
}

pub mod datagrams {
    pub mod enums;
    pub mod handler;
    pub mod packets;
    pub mod resolver;
    pub mod types;
}

pub mod events {
    pub mod handler;
    pub mod types;
}
