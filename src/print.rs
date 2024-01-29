pub trait Print {
    fn print(&mut self, value: String);
}

pub struct StdoutPrint;

impl Print for StdoutPrint {
    fn print(&mut self, value: String) {
        println!("{}", value);
    }
}
