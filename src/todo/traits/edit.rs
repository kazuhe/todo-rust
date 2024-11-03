pub trait Edit {
    fn set_to_done(&self, title: &str) {
        println!("{} を Done に設定中..", title)
    }

    fn set_to_pending(&self, title: &str) {
        println!("{} を Pending に設定中..", title)
    }
}
