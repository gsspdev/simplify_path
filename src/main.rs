fn main() {
    fn simplify_path(path: &str) -> String {
        let parts = path.split('/').filter(|s| !s.is_empty() && *s != ".");
        let mut stack: Vec<&str> = Vec::new();

        for part in parts {
            if part == ".." {
                stack.pop();
            } else {
                stack.push(part);
            }
        }

        "/".to_owned() + &stack.join("/")
    }

    let in1 = "/home/";
    let in2 = "/../";

    let out1 = simplify_path(in1);
    let out2 = simplify_path(in2);

    println!("Simplified paths: {} {}", out1, out2);
}

