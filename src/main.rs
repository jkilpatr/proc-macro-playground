#[macro_use]
extern crate serde_derive;


use std::fmt;
use std::sync::mpsc;
use std::thread;
extern crate hex;
extern crate serde;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
extern crate serde_json;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::ops::Deref;


#[derive(Copy, Clone)]
pub struct strangeSigObj(pub [u8; 20]);

impl PartialEq for strangeSigObj {
    fn eq(&self, other: &strangeSigObj) -> bool {
        self.0[..] == other.0[..]
    }
}
impl Eq for strangeSigObj {}


impl Hash for strangeSigObj {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}


impl Deref for strangeSigObj {
    type Target = [u8; 20 as usize];

    fn deref(&self) -> &[u8; 20 as usize] {
        &self.0
    }
}

impl FromStr for strangeSigObj {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(&s[2..]).map(|v| {
            let mut arr = [0u8; 20];
            arr.clone_from_slice(&v);
            strangeSigObj(arr)
        })
    }
}

impl fmt::Debug for strangeSigObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "strangeSigObj {}", self.to_string())
    }
}

impl fmt::Display for strangeSigObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0.as_ref()))
    }
}


impl Serialize for strangeSigObj {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for strangeSigObj {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}


#[derive(Serialize, Deserialize,PartialEq, Eq, Hash, Clone, Copy)]
pub struct Identity {
    pub thing: strangeSigObj
}





pub fn main() {

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        loop {
                tx1.send(Identity{thing: "0x0707070707070707070707070707070707070707".parse().unwrap()}).unwrap();
            }
    });


}
