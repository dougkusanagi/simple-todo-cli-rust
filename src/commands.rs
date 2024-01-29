use crate::storage::*;
use crate::print::*;

pub trait Command {
    fn handle(&self, storage: &impl Storage, printer: &mut dyn Print) -> i32;
}

pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    pub fn new(args: Vec<String>) -> Self {
        AddCommand { args }
    }
}

impl Command for AddCommand {
    fn handle(&self, storage: &impl Storage, printer: &mut dyn Print) -> i32 {
        let description_arg = &self.args.get(2);

        if let Some(description) = description_arg {
            storage.write(description);
            printer.print("Todo added".to_string());

            0
        } else {
            printer.print("No description provided".to_string());

            1
        }
    }
}

pub struct ListCommand {}

impl ListCommand {
    pub fn new() -> Self {
        ListCommand {}
    }
}

impl Command for ListCommand {
    fn handle(&self, storage: &impl Storage, printer: &mut dyn Print) -> i32 {
        let contents = storage.read();

        printer.print(contents);

        0
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct StorageMem {
        temp: RefCell<Vec<String>>,
    }

    impl StorageMem {
        fn new() -> Self {
            StorageMem {
                temp: RefCell::new(Vec::new()),
            }
        }
    }

    impl Storage for StorageMem {
        fn write(&self, description: &str) {
            // self.temp.push(description.to_string());
            self.temp.borrow_mut().push(description.to_string());
        }

        fn read(&self) -> String {
            self.temp.borrow().join("\n")
        }
    }

    #[test]
    fn test_add_command() {
        let args = vec![String::new(), String::from("add"), String::from("Learn Rust")];
        let command = AddCommand::new(args);
        let storage = StorageMem::new();
        let mut printer = StdoutPrint;

        assert_eq!(command.handle(&storage, &mut printer), 0);
    }

    #[test]
    fn test_add_command_missing_description() {
        let args = vec![String::new(), String::from("add")];
        let command = AddCommand::new(args);
        let storage = StorageMem::new();
        let mut printer = StdoutPrint;

        assert_eq!(command.handle(&storage, &mut printer), 1);
    }

    #[test]
    fn test_list_command() {
        let command = ListCommand::new();
        let storage = StorageMem::new();
        let mut printer = StdoutPrint;

        assert_eq!(command.handle(&storage, &mut printer), 0);
    }

    #[derive(Default)]
    struct DummyLogger(Vec<String>);

    impl Print for DummyLogger {
        fn print(&mut self, value: String) {
            self.0.push(value.to_string());
        }
    }

    #[test]
    fn test_list_command_read_contents() {
        let todo: String = String::from("Learn Rust");
        let args = vec![String::new(), String::from("add"), todo];
        let storage = StorageMem::new();
        let mut print = DummyLogger::default();

        AddCommand::new(args).handle(&storage, &mut print);
        ListCommand::new().handle(&storage, &mut print);

        assert!(print.0.get(0).unwrap().contains("Todo added"));
        assert!(print.0.get(1).unwrap().contains("Learn Rust"));
    }
}
