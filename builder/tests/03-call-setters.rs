// Generate methods on the builder for setting a value of each of the struct
// fields.
//
//     impl CommandBuilder {
//         fn executable(&mut self, executable: String) -> &mut Self {
//             self.executable = Some(executable);
//             self
//         }
//
//         ...
//     }

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

impl Command {
    fn builder() -> Self{
        Self {
            executable: String::new(),
            args: Vec::new(),
            env: Vec::new(),
            current_dir: String::new()
        }
    }

    fn executable(&mut self, new: String) {
        self.executable = new;
    }

    fn args(&mut self, new: Vec<String>) {
        self.args = new;
    }

    fn env(&mut self, new: Vec<String>) {
        self.env = new;
    }

    fn current_dir(&mut self, new: String) {
        self.current_dir = new;
    }
}

fn main() {
    let mut builder = Command::builder();
    builder.executable("cargo".to_owned());
    builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    builder.env(vec![]);
    builder.current_dir("..".to_owned());
}
