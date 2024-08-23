use serde::ser::{Serialize, Serializer, SerializeStruct};

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(input: String) -> Self {
        match input.as_str() {
            "PENDING" => Self::PENDING,
            "DONE" => Self::DONE,
            _ => panic!("input: {} not supported.", input),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}
