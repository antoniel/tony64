fn main() {
    println!("Zg== {}", encode("f"));
    println!("Zm8= {}", encode("fo"));
    println!("Zm9v {}", encode("foo"));
    println!("Zm9vYg== {}", encode("foob"));
    println!("Zm9vYmE= {}", encode("fooba"));
    println!("Zm9vYmFy {}", encode("foobar"))
}

fn encode(s: &str) -> String {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let binary_string = string_to_binary(s);
    let mut groups: Vec<String> = Vec::new();
    let mut group: String = String::new();
    for i in 0..binary_string.len() {
        group.push_str(&binary_string[i..i + 1]);
        if group.len() == 6 || i == binary_string.len() - 1 {
            groups.push(group.clone());
            group.clear();
        }
    }

    let encoded_without_padding: String = groups
        .iter()
        .map(binary_to_ascii)
        .map(|x| base64_chars.chars().nth(x as usize).unwrap())
        .collect();
    let mut encoded: String = encoded_without_padding.clone();
    while encoded.len() % 4 != 0 {
        encoded.push('=');
    }
    return encoded;
}

fn string_to_binary(s: &str) -> String {
    let mut binary: String = String::new();
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        let b = format!("{:08b}", (c as u8));
        binary.push_str(&b);
    }
    return binary;
}

fn binary_to_ascii(binary_value: &String) -> u8 {
    let mut binary_value = binary_value.clone();
    while binary_value.len() < 6 {
        binary_value.push('0');
    }
    let value = isize::from_str_radix(&binary_value, 2).unwrap();
    return value.try_into().unwrap();
}
