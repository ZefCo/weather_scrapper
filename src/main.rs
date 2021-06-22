mod path;
// user hyper::Client;

fn main() {
    // println!("Hello, world!");
    let cwd = path::getcwd()
        .expect("Incorrect permissions");
    println!("The current working directory is {}", cwd.display())
}
