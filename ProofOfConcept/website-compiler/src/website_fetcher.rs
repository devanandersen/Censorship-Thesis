use std::error::Error;
use std::path::Path;
use std::fs::read_to_string;

pub async fn get_website(url: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(&url[8..]);
    // To simulate websites adding tags, we store HTML locally and update it as the program runs.
    // This is done in individual files stored under the website name
    if path.exists() {
        Ok(read_to_string(path).unwrap())
    } else {
        let res = reqwest::get(url)
            .await?
            .text()
            .await?;

        Ok(res.as_str().to_string())
    }
}
