use reqwest;
use serde_json;

pub struct Task {}

impl Task {
    fn create() {}
}

impl Task {
    pub fn fetch(api_key: &str) {
        let endpoint = format!("{}/tasks", Task::ENDPOINT);
        let client = reqwest::blocking::Client::new();

        let resp = client.get(&endpoint).basic_auth(api_key, Some("")).send().unwrap();

        let resp_body = resp.json::<serde_json::value::Value>().unwrap();
        println!("{}", resp_body);
    }
}

impl Task {
    fn cancel() {}
}

impl Task {
    const TASK_TYPES: [&'static str; 16] = [
        "annotation",
        "audiotranscription",
        "categorization",
        "comparison",
        "cuboidannotation",
        "datacollection",
        "imageannotation",
        "lineannotation",
        "namedentityrecognition",
        "pointannotation",
        "polygonannotation",
        "segmentannotation",
        "transcription",
        "videoannotation",
        "videoboxannotation",
        "videocuboidannotation",
    ];

    const ENDPOINT: &'static str = "https://api.scale.com/v1/";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn create_task() {
        let api_key: String = env::var("SCALE_API_KEY").unwrap();
        println!("hello world");
        Task::fetch(&api_key);
        assert_eq!(2, 2);
    }
}
