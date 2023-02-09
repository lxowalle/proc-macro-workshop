// Generate a `build` method to go from builder to original struct.
//
// This method should require that every one of the fields has been explicitly
// set; it should return an error if a field is missing. The precise error type
// is not important. Consider using Box<dyn Error>, which you can construct
// using the impl From<String> for Box<dyn Error>.
//
//     impl CommandBuilder {
//         pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
//             ...
//         }
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

    fn build(self) -> Result<Self, ()>{
        Ok(self)
    }
}

fn main() {
    let mut builder = Command::builder();
    builder.executable("cargo".to_owned());
    builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    builder.env(vec![]);
    builder.current_dir("..".to_owned());

    let command = builder.build().unwrap();
    assert_eq!(command.executable, "cargo");
}
