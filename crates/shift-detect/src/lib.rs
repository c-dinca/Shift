pub struct DetectStack{
    pub node_version: Option<String>,
    pub packages: Vec<String>,
}

pub fn detect_stack() -> DetectStack {
    DetectStack{
        node_version: None,
        packages: vec![],
    }
}