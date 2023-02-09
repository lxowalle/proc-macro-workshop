// This test case should be a freebie if the previous ones are already working.
// It shows that we can chain method calls on the builder.

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

    fn executable(mut self, new: String) -> Self{
        self.executable = new;
        self
    }

    fn args(mut self, new: Vec<String>) -> Self {
        self.args = new;
        self
    }

    fn env(mut self, new: Vec<String>) -> Self {
        self.env = new;
        self
    }

    fn current_dir(mut self, new: String) -> Self {
        self.current_dir = new;
        self
    }

    fn build(self) -> Result<Self, ()>{
        Ok(self)
    }
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .current_dir("..".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}
