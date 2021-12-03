pub fn fetch(url: &str) -> String {
    let client = reqwest::blocking::Client::new();
    return client
        .get(url)
        .header(reqwest::header::COOKIE, "session=")
        .send()
        .unwrap()
        .text()
        .unwrap();
}
