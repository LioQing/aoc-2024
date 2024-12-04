/// Get the advent of code input
pub fn input(day: u8) -> String {
    dotenv::dotenv().unwrap();

    let client = reqwest::blocking::Client::new();

    const URL: &str = "https://adventofcode.com/2024/day/{day}/input";

    let input_url = URL.replace("{day}", &day.to_string());

    let request = client
        .request(reqwest::Method::GET, &input_url)
        .header(
            "Cookie",
            format!("session={}", std::env::var("SESSION").unwrap()),
        )
        .build()
        .unwrap();

    client.execute(request).unwrap().text().unwrap()
}
