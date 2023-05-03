use std::any::Any;
use std::error::Error;
use std::fmt::Display;
use std::process::Command;

pub type BoxedError = Box<dyn Error + Send + Sync>;

pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

fn exe_gen(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

// & trait obj
fn exe_trait(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

// boxed obj
fn exe_box_tr(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn t() {
        let cmd = Shell::new("ls", &[]);
        let result = cmd.run().unwrap();
        assert_eq!(result, Some(0));
    }
    #[test]
    fn exer() {
        let cmd = Shell::new("ls", &[]);
        let result = exe_gen(&cmd).unwrap();
        assert_eq!(result, Some(0));

        let result = exe_trait(&cmd).unwrap();
        assert_eq!(result, Some(0));
        let result = exe_box_tr(Box::new(cmd)).unwrap();
        assert_eq!(result, Some(0));
    }
}
