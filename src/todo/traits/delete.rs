pub trait Delete {
    fn delete(&self, title: &str) {
        println!("{} を削除中..", title);
    }
}
