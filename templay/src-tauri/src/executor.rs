pub struct ExecutorInfo {
    path: String,
    args: Vec<String>,
}

impl ExecutorInfo {
    pub fn new(path: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            path: path.into(),
            args,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

pub fn execute(info: ExecutorInfo) -> Result<i32, String> {
    let out = std::process::Command::new(&info.path)
        .args(&info.args)
        .status()
        .expect("failed to execute process");

    match out.code() {
        Some(c) => Ok(c),
        None => Err("process terminated by signal".to_string()),
    }
}
