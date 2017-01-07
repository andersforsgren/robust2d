extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/predicates.c")
        .compile("libpredicates.a");
}