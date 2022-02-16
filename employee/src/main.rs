use std::io;
use std::collections::HashMap;

fn main() {

    let mut vec = vec![("Engineering", "Sally"), ("Engineering", "Bob"), ("Sales", "Amir")];

    println!("Enter command:");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");

    
    let mut iter = buffer.split_whitespace();
    if iter.next().unwrap() == "Add" {             //Add
        let value = iter.next().unwrap();          //Name
        iter.next();                               //to
        let key = iter.next().unwrap();            //Department
        vec.push((key, value));
    }
    vec.sort();

    let mut map = HashMap::new();
    
    for (key, _value) in &vec {
        let empty_vec = Vec::new();
        map.insert(key, empty_vec);
    }  

    for (key, value) in &vec {
        let mut new_value = map.get(key).unwrap().clone();
        new_value.push(value);
        map.insert(key, new_value);
    }  
    
    let query = "Engineering";
    let val = map.get(&query).unwrap();
    println!("{} = {:?}", query, val);        

    let mut v = Vec::new();
    for (key, value) in &map {
        v.push((key,value));
    }
    v.sort_by_key(|a| a.0);
    println!("All departments = {:?}", v);
}