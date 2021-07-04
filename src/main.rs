mod common;
mod spotify;

use common::get_location;
use spotify::SpotifyRef;

// export CLIENT_ID="your client_id"
// export CLIENT_SECRET="secret"

#[tokio::main]
async fn main() {
    let mut current_song = String::new();
    let mut current_artists = Vec::new();
    let mut lat: i64;
    let mut lng: i64;

    let scope = "user-read-currently-playing";
    let redirect_uri = "http://localhost:8888/callback";

    let spotify = SpotifyRef::new(scope, redirect_uri).await;

    spotify.get_current_song(&mut current_song)
        .await;
    match current_song.as_str()
    {
        "" => println!("Not currently playing"),
        _ => println!("Currently playing: {}", current_song.to_string())
    }

    spotify.get_current_artists(&mut current_artists)
        .await;
    if !current_artists.is_empty()
    {
        println!("Artists: ");
        for artist in current_artists
        {
            println!("{}", artist.to_string());
            let tmp = get_location(&artist.to_string()).await;
            lat = tmp.0;
            lng = tmp.1;
            println!("{:?}, {:?}", lat, lng);
        }
    }
}
