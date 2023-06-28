use reqwest;
use reqwest::header::COOKIE;
use tokio;

#[tokio::main]
async fn main() {
    let input = get_advent_code_day1_input().await;
    let input = input.unwrap();
    let input: Vec<&str> = input.split('\n').collect();
    let mut all: Vec<u32> = Vec::new();
    let mut tmp: u32 = 0;
    for i in input {
        if i == "" {
            all.push(tmp);
            tmp = 0;
        }else{
            tmp += i.parse::<u32>().unwrap();
        }
    }
    println!("all: {all:?}");
    all.sort_unstable();
    let last = all.len();
    // max = 65912
    println!("top three: {} {} {}", all[last - 1], all[last - 2], all[last - 3]);
    println!("total top three: {}", all[last - 1] + all[last - 2] + all[last - 3]);
    println!("max = {}", all.iter().max().unwrap());
}

async fn get_advent_code_day1_input() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get("https://adventofcode.com/2022/day/1/input").header(COOKIE, "session=53616c7465645f5f2bcd05c6faa4114524d7c4a0d5484dd47d7db051e210c50058b511ad12ff61feea8dfc3530d253b6a2cb482ff2749283754801ee50b81e6a").send()
        .await?
        .text()
        .await?;

    Ok(body)
}
