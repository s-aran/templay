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
    #[cfg(target_os = "windows")]
    let pre_command = "cmd.exe";
    #[cfg(target_os = "windows")]
    let mut pre_args = vec!["/C".to_owned(), "start".to_owned()];
    #[cfg(target_os = "macos")]
    let pre_command = "open";
    #[cfg(target_os = "macos")]
    let mut pre_args = vec!["-Wa".to_owned()];
    // #[cfg(target_os = "linux")]

    pre_args.append(&mut info.args.clone());
    let status = tauri::api::process::Command::new(pre_command)
        .args(pre_args)
        .status()
        .unwrap();

    match status.code() {
        Some(c) => Ok(c),
        None => Err("process terminated by signal".to_string()),
    }
}
