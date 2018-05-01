use std::fs: File;

pub fn foo() -> String {
    let file = File::open("./test-submodule/hello.txt");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
