pub trait Command {
    fn execute(&mut self, command_parts: Vec<&str>);
    fn show_info(&self);
    fn name(&self) -> &str;
}
