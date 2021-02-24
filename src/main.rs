use std::io;
extern crate reqwest;
extern crate json;
extern crate regex;
extern crate itertools;
use reqwest::Client;
use regex::Regex;
use itertools::join;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let re = match Regex::new(r"birth_place\s*=\s*([^\\n]*)\\n")
    {
        Ok(re) => re,
        Err(err) => {
            println!("{}", err);
            panic!(err)
        }
    };

    let script_regex = match Regex::new(r"<[^>]*>")
    {
        Ok(re) => re,
        Err(err) => {
            println!("{}", err);
            panic!(err)
        }
    };

    let mut num_artists = String::new();
    println!("How many artists?");

    io::stdin()
        .read_line(&mut num_artists)
        .expect("Failed to read line");

    let mut num_artists: u32 = num_artists.trim().parse().expect("Please type a number!");
    let mut artists = Vec::new();
    
    
    
    while num_artists > 0 {
        println!("Next artist");

        let mut artist = String::new();

        io::stdin()
            .read_line(&mut artist)
            .expect("Failed to read line");

        artists.push(artist);
        num_artists -= 1;
    }

    let client = Client::new();


    for artist in artists {
        let location = loc(&artist, &client, &re, &script_regex);
        //match location {
            //Some(x) => println!("{}", x),
            //None => {},
        //}
        println!("{}", location.await?);
    }
    Ok(())
}

async fn loc(name: &str, client: &Client, re: &Regex, script_regex: &Regex) -> Result<String, reqwest::Error>
{
    //let mut wiki_name = name.split(" ").collect();
    let wiki_name = join(name.split(" "), "_");
    let wiki_url = &(format!("https://en.wikipedia.org/w/api.php?action=parse&page={}&format=json", wiki_name)).to_string();
    let resp = client.get(wiki_url).send().await?.text().await?;
    let entry = &json::parse(&resp).unwrap()["parse"]["text"]["*"].dump();
    let replaced = script_regex.replace_all(entry, "");
    //let entry = &pages.entries().next().unwrap().1["revisions"][0]["*"].dump();
    println!("{:?}", replaced);
    //let loc = &re.captures(entry).unwrap()[1].replace("[","").replace("]","");
    //println!("{:?}", loc);
    
    Ok("".to_string())
}
