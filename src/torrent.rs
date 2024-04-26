use serde::{Serialize, Deserialize};
use std::path::Path;
use anyhow::Context;
use hashes::Hashes;
use sha1::{Digest, Sha1};

/// A torrent file (also known as a metainfo file) 
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct Torrent {
/// URL to a "tracker"
pub announce: String,    

pub info: Info
}


impl Torrent {
    pub async fn read(file: impl AsRef<Path>) -> anyhow::Result<Self> {
        let dot_torrent = std::fs::read(file).context("read torrent file")?;
        let t: Torrent = serde_bencode::from_bytes(&dot_torrent).context("parse torrent file")?;
        Ok(t)
    }

    pub fn info_hash(&self) -> [u8; 20] {
        let info_encoded = serde_bencode::to_bytes(&self.info)
                .expect("re encoded info section is fine");
        let mut hasher = Sha1::new();
        hasher.update(&info_encoded);
        hasher
        .finalize()
        .try_into()
        .expect("GenericArray<_, 20> == [_; 20]")
    }

    pub fn length(&self) -> usize {
        match &self.info.keys {
            Keys::SingleFile { length } => *length,
            Keys::MultiFile { files } => files.iter().map(|file| file.length).sum(),
        }
    }
}

///that describes the file(s) of the torrent. 
/// There are two possible forms: one for the case of a 'single-file' t
/// orrent with no directory structure, and one for the case of a 'multi-file' torrent (see below for details)
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct Info {
    ///size of the file in bytes, for single-file torrents 
    /// suggested name to save the file / directory as
    pub name: String,
    ///piece length: number of bytes in each piece
    #[serde(rename="piece length")]
    pub plength: usize,
    /// concatenated SHA-1 hashes of each piece
    pub pieces: Hashes,

    #[serde(flatten)]    
    pub keys:Keys
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
#[serde(untagged)]
pub enum Keys {
    SingleFile {
        length:usize
    },
    MultiFile {
        files:Vec<File>       
    }
}
    
#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct File {
    /// length of the file in bytes (integer)
    length:usize,
    /// a list containing one or more string elements that together represent the path and filename. 
    /// Each element in the list corresponds to either a directory name or (in the case of the final element) the filename.
    path: Vec<String>
}


mod hashes {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Hashes(pub Vec<[u8; 20]>);
    struct HashesVisitor;

    impl<'de> Visitor<'de> for HashesVisitor {
        type Value = Hashes;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte string whose length is a multiple of 20")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() % 20 != 0 {
                return Err(E::custom(format!("length is {}", v.len())));
            }
            // TODO: use array_chunks when stable
            Ok(Hashes(
                v.chunks_exact(20)
                    .map(|slice_20| slice_20.try_into().expect("guaranteed to be length 20"))
                    .collect(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Hashes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(HashesVisitor)
        }
    }

    impl Serialize for Hashes {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let single_slice = self.0.concat();
            serializer.serialize_bytes(&single_slice)
        }
    }
}