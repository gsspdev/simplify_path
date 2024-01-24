fn main() {
    pub fn simplify_path(path: String) -> String {
        let mut input: String = path.clone();
        let mut input_lngth: usize = path.len();

        pub struct Trailing<'a> {
            slash: Option<char>,
            double_dot: Option<&'a str>
        }

        let trailing = Trailing {
            slash: Some('/'),
            double_dot: Some("..")
        };

        let end_slash = trailing.slash.unwrap();
        let end_dots = trailing.double_dot.unwrap().chars().collect();
        
        let mut output: Some(<Vec<char>>) = Some(vec![char]) input.chars().collect(); 
        output: Some(<Vec<char>>) = output.filter(end_slash);
        output: Some(<Vec<char>>) = output.filter(end_dots);
        let final_output: String = output.into_iter().collect();

        println!("output: {}", final_output);
        return final_output;
    }
    
    let in1: &str = "/home";
    let in2: &str = "/../";

    let out1 = simplify_path(in1.to_string());
    let out2 = simplify_path(in2.to_string());

    println!("Simplified paths: {} {}", out1, out2);

}
