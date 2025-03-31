/**
 * main librray core structures
 */
pub struct FunParser;
pub struct ClassParser;
enum FunT { Bool = 0, Int32 = 1, Int64 = 2, Int8 = 3, Float = 4 }

/** all error code gnerators code erros  */
enum ErrCode {

}


/// ------------------------------------

impl FunParser {
    pub fn parse_signature(&self, code: &str) -> Option<String> {
        Some(code.lines().next()?.trim().to_string())
    }

    pub fn parse_return_type(&self, code: &str) -> Option<String> {
        let first_line = code.lines().next()?;
        Some(first_line.split_whitespace().next()?.to_string())
    }

    pub fn parse_function_name(&self, code: &str) -> Option<String> {
        Some("example_function".to_string()) 
    }

    pub fn parse_parameters(&self, code: &str) -> Vec<(String, String)> {
        vec![
            ("int".to_string(), "param1".to_string()), 
            ("float".to_string(), "param2".to_string())
        ]
    }

    pub fn validate_function(&self, code: &str) -> bool {
        code.contains("(") && code.contains(")")
    }
}
  
impl  ClassParser {
    
}
