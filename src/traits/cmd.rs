pub trait Cmd {
    fn run(self) -> Result<(), String>;
}
