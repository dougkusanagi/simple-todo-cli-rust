use std::fs;
use std::io::Write;

pub trait Storage {
    fn write(&self, description: &str);

    fn read(&self) -> String;
}

pub struct StorageTxt {
    filepath: String,
}

impl StorageTxt {
    pub fn new(filepath: String) -> Self {
        StorageTxt { filepath }
    }
}

impl Storage for StorageTxt {
    fn write(&self, description: &str) -> () {
        let mut file = fs::OpenOptions
            ::new()
            .write(true)
            .append(true)
            .open(&self.filepath)
            .expect("File not found");

        writeln!(file, "{description}").expect("File not writable");
    }

    fn read(&self) -> String {
        let contents = fs::read_to_string("storage.txt").expect("File not found");

        contents
    }
}
