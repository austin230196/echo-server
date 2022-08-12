use std::{env, process, fs, path::Path};




pub mod helpers {
    use super::*;
    pub fn get_env(){
        env::set_current_dir("./");
        let path = Path::new(".env");
        let file_content = fs::read(path).unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(1);
        });
        let content = String::from_utf8_lossy(&file_content);
        let set_env = |e: String| {
            if let [key, value] = e.split("=").map(|f| f.to_owned()).collect::<Vec<String>>().as_slice() {
                env::set_var(key.trim(), value.trim());
            }else {
                return;
            }
        };
        for c in content.split("\n").collect::<Vec<&str>>() {
            set_env(c.to_owned())
        }
    }
}