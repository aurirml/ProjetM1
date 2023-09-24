fn display_tab_int(tab: &[i32]){
    let mut i=0;

    let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    for value in v{
        print!("\t {}", value);
    }
    println!("");
    println!("");
    while i<100 {
        print!("{} \t", ((i+ 10)/10));
        for _j in 0..10{
            print!("[{}] \t",tab[i]);
            i +=1; 
        }
        println!(" \n");
      while i<100 {
        for _j in 0..10 {
            print!("[{}]",tab[i]);
            i +=1; 
        }
        println!(" ");
    }
    println!(" ");
}

fn display_tab_str(tab: &[&str]){
    let mut i=0;
    let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    for value in v{
        print!("\t {}", value);
    }
    println!("");
    println!("");
    while i<100 {
        print!("{} \t", ((i+ 10)/10));
        for _j in 0..10{
            print!("[{}] \t",tab[i]);
            i +=1; 
        }
        println!(" \n");
    while i<100 {
        for _j in 0..10 {
            print!("[{}]",tab[i]);
            i +=1; 
        }
        println!(" ");
    }
    println!(" ");
}


fn main() {
    let _tab_j1int: [i32;100]=[0;100];
    let _tab_j1str: [&str;100]=["x";100];
    let _tab_j2int: [i32;100]=[0;100];
    let _tab_j2str: [&str;100]=[" ";100];
    display_tab_int(&_tab_j1int);
    display_tab_str(&_tab_j1str);
}