use bittorrent_starter_rust::decode_bencoded::decode_bencoded_value;
#[test]
fn test_decode_bencoded_integer() {
    let encoded_int = "i25e";
    let (value, _) = decode_bencoded_value(encoded_int);
    //println!("{value}");
    assert_eq!(value, 25);
}

#[test]
fn test_decoded_bencoded_string() {
    let encoded_str = "5:hello";
    let (value, _) = decode_bencoded_value(encoded_str);   
    assert_eq!(value, "hello".to_string());
    println!("{value}");
}

#[test] 
fn test_decode_bencoded_list() {
    let encoded_str = "l4:spam4:eggse";
    let value = decode_bencoded_value(encoded_str).0;
    assert_eq!(value[0] , "spam");
    assert_eq!(value[1], "eggs");
}


#[test] 
fn test_decode_bencoded_dict() {
    let encoded_str = "d9:publisher3:bob17:publisher-webpage15:www.example.com18:publisher.location4:homee";
    let value = decode_bencoded_value(encoded_str).0;
    assert_eq!(value["publisher"] , "bob");
    assert_eq!(value["publisher.location"] , "home");
}