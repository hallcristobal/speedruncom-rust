pub fn vec_to_parameters(vec: &Vec<String>) -> String {
    if vec.is_empty() {
        String::new()
    } else {
        format!("?{}", vec.join("&"))
    }
}

pub fn string_to_parameters(parameters: &str) -> String {
    if parameters.is_empty() {
        String::new()
    } else {
        format!("?{}", parameters)
    }
}
