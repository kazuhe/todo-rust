pub enum TaskStatus {
    Done,
    Pending,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match self {
            Self::Done => "Done".to_string(),
            Self::Pending => "Pending".to_string(),
        }
    }
}
