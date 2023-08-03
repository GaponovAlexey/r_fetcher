pub mod postal {
    use once_cell::sync::Lazy;
    
    static C1: Lazy<String> = Lazy::new(|| "hi from constant".to_string());

    pub struct Contract {
        pub pr: Vec<Box<dyn Project>>,
    }

    pub trait Project {
        fn one_p(&self) {
            println!("from Project");
        }
    }

    pub struct One {}
    pub struct Two {}
    pub struct Three {}

    impl Project for One {
        fn one_p(&self) {
            println!("from one");
            println!("time### :{:?}", C1.to_string())
        }
    }
    impl Project for Two {
        fn one_p(&self) {
            println!("from two");
            println!("const :{:?}", C1.to_string())
        }
    }
    impl Project for Three {}
}
