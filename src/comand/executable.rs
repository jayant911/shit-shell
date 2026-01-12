use std::process;
use std::process::Stdio;

use pathsearch::find_executable_in_path;

pub struct Executable {
    pub name: String,
    pub args: Vec<String>,
}

impl Executable {
    pub fn execute(&self) {
        let args = self.args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        match find_executable_in_path(&self.name) {
            Some(path) => {
                let mut child = process::Command::new(path)
                    .args(args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                child.wait().expect("failed to wait on child");
            }
            None => {
                eprintln!("{}: command not found", self.name);
            }
        }
    }
}
