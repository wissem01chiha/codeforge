fn main() {
    println!("Hello, world!");
}


pub trait TestGenerator {
    fn generate(&self, function_name: &str) -> String;
}

pub struct GoogleTestGenerator;
impl TestGenerator for GoogleTestGenerator {
    fn generate(&self, function_name: &str) -> String {
        format!("TEST({}, {}) {{ /* TODO */ }}", function_name, "TestCaseName")
    }
}

pub struct PytestGenerator;
impl TestGenerator for PytestGenerator {
    fn generate(&self, function_name: &str) -> String {
        format!("def test_{}():\n    pass", function_name)
    }
}

// Factory Function
pub fn create_generator(language: &str) -> Box<dyn TestGenerator> {
    match language {
        "C++" => Box::new(GoogleTestGenerator),
        "Python" => Box::new(PytestGenerator),
        _ => panic!("Unsupported language!"),
    }
}
