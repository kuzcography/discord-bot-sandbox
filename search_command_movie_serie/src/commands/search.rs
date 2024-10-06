use serenity::model::application::{ResolvedOption, ResolvedValue};

pub fn run_search(options: &[ResolvedOption]) -> String {
    let mut name = String::new();
    let mut platform = String::new();
    
    for option in options {
        match &option.value {
            ResolvedValue::String(value) if option.name == "name" => {
                name = value.to_string();
            }
            ResolvedValue::String(value) if option.name == "platform" => {
                platform = value.to_string();
            }
            _ => {}
        }
    }
    
    match platform.as_str() {
        "empirstream" => {
            
            "https://empire-streamz.fr/".to_string()
        },
        "senpaistream" => {
            
            format!("https://senpai-stream.net/search/{}", name).to_string()},
        "astreaming" => {
            
            format!("https://astreaming.eu/?s={}", name).to_string()},
        "papadustream" => {
            
            "https://papadustream.my/".to_string()},
        _ => "Searching error".to_string(),  
    }
}
