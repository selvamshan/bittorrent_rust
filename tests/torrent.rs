use bittorrent_starter_rust::torrent::{self, Torrent};

///announce: "http://bittorrent-test-tracker.codecrafters.io/announce"

#[tokio::test]
async fn test_track_url() {    
    let t = Torrent::read("sample.torrent").await.expect("file read errro");
    
    assert_eq!(t.announce, "http://bittorrent-test-tracker.codecrafters.io/announce");
}   
