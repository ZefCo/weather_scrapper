mod paths;
mod json_importer;
// user hyper::Client;

fn main() {
    // println!("Hello, world!");
    let cwd = paths::getcwd()
        .expect("Incorrect permissions");
    // println!("The current working directory is {}", cwd.display());

    let json_file = paths::sub_file(cwd, "jsons_file", "location.json");
    println!("The path to the location file is {}", json_file.display())

    // paths::sub_file(cwd, "jsons_files", "location.json")
}
