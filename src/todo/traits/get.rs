pub trait Get {
    fn get(&self, title: &str) {
        println!("{} を取得中..", title)
    }
}
