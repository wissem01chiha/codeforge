
pub trait FunParser{

}
pub trait ClassParser {

}


// basic genrator for language specific routines (functions , classes )
pub trait TestGenerator {
    fn generate(&self, function_name: &str) -> String;
}

// Factory Function
pub fn create_generator(language: &str) -> Box<dyn TestGenerator> {
    match language {
        "C++" => Box::new(GoogleTestGenerator),
        "Python" => Box::new(PytestGenerator),
        _ => panic!("Unsupported language!"),
    }
}


// google test plugin gnerator 
// by defut it gnerate all possible template tests and 
// corrponding documenttaion of each test case
pub struct GoogleTestGenerator;

impl TestGenerator for GoogleTestGenerator {
    fn generate(&self, function_name: &str) -> String {
        format!("TEST({}, {}) {{ /* TODO */ }}", function_name, "TestCaseName")
    }
}

// basic types can be passed to a function, declraitves, 
enum fun_t {
    bool,
    int32,
    int64,
    int8,
    float
}