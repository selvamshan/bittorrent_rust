#[allow(unused_imports)]
//use serde_json;
//use anyhow::Context;
use clap::Parser;

use bittorrent_starter_rust::decode_bencoded::decode_bencoded_value;
use bittorrent_starter_rust::command::{Args, Command};
use bittorrent_starter_rust::torrent::{self, Torrent};
use bittorrent_starter_rust::tracker:: TrackerResponse;

// Available if you need it!
// use serde_bencode



// Usage: your_bittorrent.sh decode "<encoded_value>"
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let args = Args::parse();

    match args.command {
        Command::Decode { value } => {
            let v = decode_bencoded_value(&value).0;
            //let v  = serde_bencode::from_str(&value);           
            println!("{}", v);
        }
        Command::Info { torrent } => {
            // let dot_torrent = std::fs::read(torrent).context("read torrent file")?;
            // let t: Torrent = serde_bencode::from_bytes(&dot_torrent).context("parse torrent file")?;
            let t  = Torrent::read(torrent).await?;
            //println!("{t:?}");
            println!("Tracker URL: {}", t.announce);            
            let length = if let torrent::Keys::SingleFile{length} = t.info.keys  {
                length
            } else {
                todo!()
            };
            println!("Length: {}", length);
            let info_hash = t.info_hash();
            //println!("Info Hash: {:?}", info_hash);
            println!("Info Hash: {}", hex::encode(&info_hash));
            println!("Piece Length: {}", t.info.plength);
            println!("Piece Hashes:");
            for hash in t.info.pieces.0 {
                println!("{}", hex::encode(&hash));
            }

        }
        Command::Peer { torrent } => {
            let t  = Torrent::read(torrent).await?;          
            println!("Tracker URL: {}", t.announce);            
            let length = if let torrent::Keys::SingleFile{length} = t.info.keys  {
                length
            } else {
                todo!()
            };
            println!("Length: {}", length);
            let info_hash = t.info_hash();
            let tracker_response = TrackerResponse::query(&t, info_hash).await?;
            println!("Tracker response {:?}", tracker_response);
            for peer in &tracker_response.peers.0 {
                println!("{}:{}", peer.ip(), peer.port());
            }

        }
    };

    Ok(())
}
