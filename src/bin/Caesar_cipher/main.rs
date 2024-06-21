

fn main (){
    let plaintext = "Hello World";
    let enc_text =encryption(plaintext);
    let dec_text =decryption(enc_text.as_str());
    println!("plaintext:{}", plaintext);
    println!("enc_text:{}", enc_text);
    println!("dec_text:{}", dec_text);
}

fn shift(text:&str, shift_num:i8) -> String {
    let mut result = String::new();

    for ch in text.chars() {
        if ch.is_alphabetic(){
            let base = if ch.is_ascii_lowercase() { b'a' } else { b'A' } as i8;
            let shifted_ch = ((ch as i8 - base + shift_num) % 26 + base) as u8 ;
            result.push(shifted_ch as char);
        }else {
            result.push(ch);
        }
}
    result
}
fn encryption(text:&str) -> String {
    shift(text,3)
}
fn decryption(text:&str) -> String {
    shift(text,-3)
}