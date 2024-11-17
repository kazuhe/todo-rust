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

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "Done" => Self::Done,
            "Pending" => Self::Pending,
            _ => panic!("入力された {} はサポートしていません", input_string),
        }
    }
}
