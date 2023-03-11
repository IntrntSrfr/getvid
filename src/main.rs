
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let platform: Platform;

    match detect_platform(args[1].clone()) {
        Ok(plt) => platform = plt,
        Err(_) => {println!("not ok!!!"); std::process::exit(1);}
    }

    println!("platform: {:?}", platform);
}

fn detect_platform(inp: String)->Result<Platform, String>{
    if inp.starts_with("https://twitter.com") {return Ok(Platform::Twitter)}
    else if inp.starts_with("https://reddit.com") { return Ok(Platform::Reddit)}
    return Err("platform not supported, or link format may be unknown".to_string());
}

fn fetch_video(){}
fn fetch_twitter_video(){}
fn fetch_reddit_video(){}
fn save_video(){}

#[derive(Debug)]
enum Platform {
    Twitter,
    Reddit
}
