fn main() {
    pub fn simplify_path(path: String) -> String {

        #[derive(Debug)]
        pub struct Trailing<'a> {
            slash: Option<char>,
            double_dot: Option<&'a str>
        }
        let trailing = Trailing {
            slash: Some('/'),
            double_dot: Some("..")
        };
        println!("{} {}", 
            format!("{}", trailing.slash.unwrap()), 
            format!("{}", trailing.double_dot.unwrap()));        // let trail_len: usize = trailing.flatten().len();
                                                             //
        let mut input: String = path.clone();
        let mut input_lngth: usize = path.len();

        let trailing = Trailing {
            slash: Some('/'),
            double_dot: Some("..")
        };
        
        if input.chars().last() == trailing.slash || trailing.double_dot.is_some() {
            input.pop();
            println!("{}", input);
        }
        return input;
    }
    
    let in1: &str = "/home";
    let in2: &str = "/../";

    let out1 = simplify_path(in1.to_string());
    let out2 = simplify_path(in2.to_string());

    println!("Simplified paths: {} {}", out1, out2);

}
