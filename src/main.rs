use std::fs;
use std::env;

fn main() -> Result<(), Box <dyn std::error::Error>>{
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let md_file = &args[2];
    println!("url: {}", url);
    println!("file_name: {}", md_file);

    // let url = "https://www.rust-lang.org/";
    // let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(md_file, md.as_bytes())?;
    println!("converted markdown has been saved in {}.", md_file);

    Ok(())
}
