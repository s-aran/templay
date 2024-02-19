use std::collections::HashMap;

use crate::executor::{self, ExecutorInfo};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArgType {
    CurrentDirectory,
    FilePath,
    FileName,
    FileExtension,
    LineNumber,
}

pub struct ArgParams {
    params: HashMap<ArgType, String>,
}

impl ArgParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, ArgType, String> {
        self.params.iter()
    }

    pub fn set_current_directory(&mut self, current_directory: impl Into<String>) {
        self.params
            .insert(ArgType::CurrentDirectory, current_directory.into());
    }

    pub fn set_file_path(&mut self, file_path: impl Into<String>) {
        self.params.insert(ArgType::FilePath, file_path.into());
    }

    pub fn set_file_name(&mut self, file_name: impl Into<String>) {
        self.params.insert(ArgType::FileName, file_name.into());
    }

    pub fn set_file_extension(&mut self, file_extension: impl Into<String>) {
        self.params
            .insert(ArgType::FileExtension, file_extension.into());
    }

    pub fn set_line_number(&mut self, line_number: u32) {
        self.params
            .insert(ArgType::LineNumber, format!("{}", line_number));
    }
}

pub struct ExternalEditor {
    name: String,
    command: String,
    args: String,
}

impl ExternalEditor {
    pub fn new(
        name: impl Into<String>,
        command: impl Into<String>,
        args: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            args: args.into(),
        }
    }

    fn arg_type_to_str(arg_type: &ArgType) -> &'static str {
        match arg_type {
            ArgType::CurrentDirectory => "CurrentDirectory",
            ArgType::FilePath => "FilePath",
            ArgType::FileName => "FileName",
            ArgType::FileExtension => "FileExtension",
            ArgType::LineNumber => "LineNumber",
        }
    }

    fn arg_type_to_str_with_bracket(arg_type: &ArgType) -> String {
        format!("{{{}}}", Self::arg_type_to_str(&arg_type))
    }

    fn replace_args(&self, params: ArgParams) -> String {
        // replace "-a {fileName}" -> "-a external_editor.rs"
        let mut args = self.args.clone();

        for iter in params.iter() {
            let arg_type_str = Self::arg_type_to_str_with_bracket(iter.0);
            let arg_value = iter.1;

            args = args.replace(&arg_type_str, arg_value);
        }

        args
    }

    pub fn execute(&self, params: ArgParams) {
        let args = self
            .replace_args(params)
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        println!("execute: {} {:?}", self.command.to_owned(), args);
        let info = ExecutorInfo::new(self.command.to_owned(), args);

        executor::execute(info);
    }
}
