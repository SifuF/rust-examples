use std::collections::HashMap;

fn main() {

    let mut v = vec![2, 1, 3, 4, 4, 4, 5, 6, 9, 10, 20, 20, 20, 20];
    v.sort();
    let median = &v[&v.len()/2];
    println!("median = {:?}", median);

    let mut map = HashMap::new();
    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (key, value) in &map {
        if *value > max {
            max = *value;
            mode = **key;
        }
    }
    println!("mode = {}", mode);
}
