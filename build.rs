

extern crate gcc;

fn main() {
    let mut cfg = gcc::Config::new();
    cfg.file("src/task_info.c");
    cfg.compile("libtask_info.a");
}
