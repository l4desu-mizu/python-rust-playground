use core::execute_python;

fn main() {
    println!(
        "{}",
        execute_python().expect("Expected to properly run though")
    );
}
