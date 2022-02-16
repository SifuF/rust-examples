use std::collections::HashMap;

fn main() {

    let mut v1 = vec!["Sally", "Bob"];
    v1.sort();
    let mut v2 = vec!["Amir"];
    v2.sort();

    let mut map = HashMap::new();

    map.insert(String::from("Sales"), &v2);
    map.insert(String::from("Engineering"), &v1);
    
    let query = String::from("Engineering");
    println!("{} = {:?}", &query, map.get(&query).unwrap());        

    let mut v = Vec::new();
    for (key, value) in &map {
        v.push((key,value));
    }
    v.sort_by_key(|a| a.0);
    println!("All departments = {:?}", v);
}