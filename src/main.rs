use std::collections::HashMap;
use std::fs;
use std::path::Path;

// fn get_applications(dir: &Path) -> HashMap<&str, str> {
//     let mut applications = HashMap::new();
// }

fn main() {
    let paths = env!("PATH").split(";");
    for path in paths {
        // find all "applications" in the directory
        fs::read_dir(path);
        // for file in fs::read_dir(path)? {
        //     let file = file?;
        // }
    }
}
