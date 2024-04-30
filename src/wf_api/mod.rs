use serde::de::DeserializeOwned;

pub mod manifest;
pub mod weapon;

const CONTENT_SERVER: &str = "http://content.warframe.com/PublicExport/Manifest";

fn download_json<T: DeserializeOwned>(url: &str) -> T {
    let body = reqwest::blocking::get(url)
        .expect(&format!("Could not download from {url}"))
        .text()
        .expect(&format!("Could not download from {url}"));
    serde_json::from_str::<T>(&body.replace("\r", "\\r").replace("\n", "\\n"))
        .expect("Could not deserialize response body")
}

fn content_server(path: &str) -> String {
    format!("{CONTENT_SERVER}/{path}")
}
