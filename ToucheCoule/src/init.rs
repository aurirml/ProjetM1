pub(crate) fn display_tab_int(tab: &[i32]){
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
    }
    println!(" ");
}

pub(crate) fn display_tab_str(tab: &[&str]){
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
    }
    println!(" ");
}