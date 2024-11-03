pub trait Create {
    fn create(&self, title: &str) {
        println!("{} を作成中..", title)
    }
}
