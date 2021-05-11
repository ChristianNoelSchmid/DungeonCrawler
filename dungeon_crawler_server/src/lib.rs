pub mod events {
    pub mod handler;
    pub mod types;
}

pub mod state {
    pub mod handler;
    pub mod snapshot;
    pub mod stats;
    pub mod traits;
    pub mod types;

    pub mod ai {
        pub mod ai_goblin;
        pub mod ai_package_manager;
        pub mod ai_packages;
    }

    pub mod transforms {
        pub mod positioner;
        pub mod transform;
        pub mod vec2;
    }

    pub mod actors {
        pub mod monsters;
        pub mod players;
    }
}

pub mod astar;
