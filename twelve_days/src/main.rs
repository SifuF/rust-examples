fn main() {
    let mut lyrics = Vec::<String>::new();
    lyrics.push(String::from("On the first day of Christmas my true love sent to me")); 
    lyrics.push(String::from("12 drummers drumming"));
    lyrics.push(String::from("Eleven pipers piping"));
    lyrics.push(String::from("Ten lords a-leaping"));
    lyrics.push(String::from("Nine ladies dancing"));
    lyrics.push(String::from("Eight maids a-milking"));
    lyrics.push(String::from("Seven swans a-swimming"));
    lyrics.push(String::from("Six geese a-laying"));
    lyrics.push(String::from("Five golden rings"));
    lyrics.push(String::from("Four calling birds"));
    lyrics.push(String::from("Three French hens"));
    lyrics.push(String::from("Two turtle-doves"));
    lyrics.push(String::from("And a partridge in a pear tree")); 
    
    let iter = 12;
    let mut x = 0;

    while x<12 {
        
        print!("{}", &lyrics[0][0..7]); 
        print!("{}", x+1);
        
        if x==0 { print!("st "); } 
        else if x==1 { print!("nd "); } 
        else if x==2 { print!("rd "); }  
        else { print!("th "); }  

        println!("{}", &lyrics[0][13..lyrics[0].len()]);
        
        for i in (iter-x)..lyrics.len() {
            if (iter-x)==12 {
                println!("{}", &lyrics[12][4..]);
            }
            else {
                println!("{}", lyrics[i]);
            }
        }
        
        println!(" ");
        x+=1;
    }
}
