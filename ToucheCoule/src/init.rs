pub(crate) fn display_tab_int() -> Vec<Vec<i32>> {
    let matrix = vec![vec![0; 10]; 10];

    for row in &matrix {
        for &element in row {
            print!("{} ", element);
        }
        println!();
    }

    matrix 
}

/* 
pub(crate) fn display_tab_str(tab: &[&str]) {
    let v = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

    // Afficher la première ligne avec les lettres
    for value in &v {
        print!("\t{}", value);
    }
    println!("\n");

    let mut i = 0;
    while i < 100 {
        print!("{}\t", (i + 10) / 10);

        for _j in 0..10 {
            print!("[{}]\t", tab[i as usize]);
            i += 1;
        }
        println!("\n");
    }
    println!("");
}
*/