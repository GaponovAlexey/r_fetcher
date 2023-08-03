pub mod some_mod {
    use chrono::Utc;

    pub struct Project {
        pub members: Vec<Box<dyn TeamMembers>>,
    }
    pub trait TeamMembers {
        fn do_task(&self) {
            println!("do task = {:?}", Utc::now())
        }
    }

    pub struct Qa {}

    impl TeamMembers for Qa {
        fn do_task(&self) {
            println!("do qa task = {:?}", Utc::now())
        }
    }

    #[derive(Debug)]
    pub struct Dev {
        lang: String,
        exp: i32,
        tasks: Vec<String>,
    }

    impl TeamMembers for Dev {
        fn do_task(&self) {
            println!("do dev task = {:?}", Utc::now())
        }
    }

    impl Dev {
        pub fn new(lang: String) -> Dev {
            Dev { lang, exp: 2, tasks: Vec::new() }
        }
        pub fn change_lang(&mut self, new_lang: String) {
            self.lang = new_lang.to_string();
        }
        pub fn list_task(&self) -> &Vec<String> {
            &self.tasks
        }
        pub fn add_task(&mut self, task: String) {
            self.tasks.push(task);
        }
    }
}
