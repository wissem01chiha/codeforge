
mod lib;

fn main() {
    println!("Hello, world!");
}





pub struct PytestGenerator;
impl TestGenerator for PytestGenerator {
    fn generate(&self, function_name: &str) -> String {
        format!("def test_{}():\n    pass", function_name)
    }
}


