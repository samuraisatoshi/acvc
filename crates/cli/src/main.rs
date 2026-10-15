#![forbid(unsafe_code)]

fn main() {
    if let Err(err) = acvc_cli::run() {
        let mut context = serde_json::Map::new();
        for (key, value) in &err.context {
            context.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        eprintln!(
            "{}",
            serde_json::json!({ "error": err.message, "context": context })
        );
        std::process::exit(1);
    }
}
