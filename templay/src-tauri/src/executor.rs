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

pub fn execute(info: ExecutorInfo) {
    #[cfg(target_os = "windows")]
    let pre_command = "cmd.exe";
    #[cfg(target_os = "windows")]
    let mut pre_args = vec!["/C".into(), "start".into(), r#""#.into()];
    #[cfg(target_os = "macos")]
    let pre_command = "open";
    #[cfg(target_os = "macos")]
    let mut pre_args = vec!["-a".to_owned()];
    // #[cfg(target_os = "linux")]

    pre_args.push(info.path);
    pre_args.append(&mut info.args.clone());
    tauri::api::process::Command::new(pre_command)
        .args(pre_args)
        // .status()
        .spawn()
        .unwrap();
}
