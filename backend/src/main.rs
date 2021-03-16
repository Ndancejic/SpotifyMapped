extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-currently-playing")
        .redirect_uri("http://localhost:8888/callback")
        .build();

    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let current_song = spotify
                .current_playing(None)
                .await;

            println!("{:?}", current_song);
        }
        None => println!("auth failed"),
    };
}
