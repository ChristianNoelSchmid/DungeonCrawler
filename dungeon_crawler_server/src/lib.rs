pub mod events {
    pub mod commands {
        pub mod cmd;
        pub mod combat;
        pub mod status;
        pub mod sync;
    }
    pub mod manager;
}

pub mod state {
    pub mod actor;
    pub mod manager;
    pub mod monsters;
    pub mod players;
    pub mod snapshot;
    pub mod stats;
    pub mod traits;
    pub mod types;

    pub mod ai {
        pub mod ai_package_collections;
        pub mod ai_package_manager;
        pub mod ai_packages;
    }

    pub mod transforms {
        pub mod transform;
        pub mod vec2;
        pub mod world_stage;
    }
}

pub mod astar;
