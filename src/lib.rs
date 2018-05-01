use std::fs::File;
use std::io::Read;

pub fn foo() -> String {
    let mut file = File::open("./test-submodule/hello.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
