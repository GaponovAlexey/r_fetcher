pub mod development {
    pub struct Development {
        pub lang: String,
        salary: i32,
    }

    impl Development {
        pub(crate) fn new(land: String) -> Development {
            Development { lang: land, salary: 1000000000 }
        }
    }

    pub mod dev {
        pub fn add_task() {
            println!("Hi from task")
        }
        fn complete_task() {}
    }

    pub mod qa {
        pub fn find_bug() {
            super::super::development::dev::add_task();
        }
        fn resolve_bug() {}
    }
}

pub fn make_project() {
    // dev::add_task();
    let d1 = development::Development::new("Rust".to_string());
    println!("{:?}", d1.lang)
    // self::development::dev::add_task();

    // development::dev::add_task();
}
