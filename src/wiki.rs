extern crate reqwest;
extern crate json;
extern crate regex;
extern crate itertools;
use reqwest::Client;
use regex::Regex;
use itertools::join;

pub async fn get_birthplace(artist: &str)
{
    let birth_place_regex = match Regex::new(r"birth_place\s*=\s*([^\\n]*)\\n")
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

    let location = get_wiki_regex(&artist, &birth_place_regex, &script_regex);
    println!("{:?}", location.await);
}

async fn get_wiki_regex(name: &str, re: &Regex, script_regex: &Regex) -> Result<String, reqwest::Error>
{
    let client = Client::new();

    let wiki_name = join(name.split(" "), "_");
    let wiki_url = (format!("https://en.wikipedia.org/w/api.php?action=parse&page={}&format=json", wiki_name)).to_string();
    let res = match client
        .get(&wiki_url)
        .send()
        .await
        {
            Ok(re) => re,
            Err(err) => {
                println!("{}", err);
                panic!(err)
            }
        };

    let resp = match res
        .text()
        .await
        {
            Ok(re) => re,
            Err(err) => {
                println!("{}", err);
                panic!(err)
            }
        };

    let entry = &json::parse(&resp).unwrap()["parse"]["text"]["*"].dump();
    let replaced = script_regex.replace_all(entry, "");
    println!("{:?}", replaced);

    return
    Ok("".to_string())
}
