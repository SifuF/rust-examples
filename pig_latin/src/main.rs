fn main() {
    
    let mut string = String::from("first");
    //let mut string = String::from("apple");
    let s = string.chars().next().unwrap();

    if s == 'a' || s == 'e' || s == 'i' || s == 'o' || s == 'u' {
        string.push_str("-hay"); 
    }
    else {
        string.push_str("-");
        string.push_str(&s.to_string());
        string.push_str("ay");
        string = String::from(&string[1..string.len()]);
    }

    println!("{}", &string);

}
