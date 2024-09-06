pub mod tr {
    pub struct Newspaper {
        pub title: String,
        pub content: String,
    }

    pub trait Summary {
        fn summary(&self) -> String;
        fn intro_summary(&self) {
            println!("Read more on {}", self.summary());
        }
    }

    impl Summary for Newspaper {
        fn summary(&self) -> String {
            format!("{}", self.content)
        }
    }

}