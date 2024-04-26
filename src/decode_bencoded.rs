

#[allow(unused_imports)]
use serde_json;
pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next(){
        Some('i') => {
            if let Some((n, rest)) =
                encoded_value
                    .split_at(1)
                    .1
                    .split_once('e')
                    .and_then(|(digits, rest)| {
                        let n = digits.parse::<i64>().ok()?;
                        Some((n, rest))
                    })
            {
                return (n.into(), rest);
            }
        }        
        Some('1'..='9') => {
            if let Some((len, rest)) = encoded_value.split_once(':') {
                if let Ok(len) = len.parse::<usize>() {
                    return (rest[..len].to_string().into(), &rest[len..]);
                }
            }
        }
        Some('l') => {
            let mut values = Vec::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with('e') {
                let (v, reminder) = decode_bencoded_value(rest);
                values.push(v);
                rest = reminder;
            }
            return (values.into(), &rest[1..])
        }
        Some('d') => {
            let mut dict = serde_json::Map::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with('e') {
                let (k, reminder) = decode_bencoded_value(rest);
                let k = match k {
                    serde_json::Value::String(k) => k,
                    k => {
                        panic!("dict key must be string not {k:?}")
                    }                
                };
                let (v, reminder) = decode_bencoded_value(reminder);
                dict.insert(k, v);
                rest = reminder;
            }

            return (dict.into(), &rest[1..]);
        }

        _ => {}
       
    }
    panic!("Unhandled encoded value: {}", encoded_value)
    
}

