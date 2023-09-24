use std::io;

pub(crate) fn futurcoup() -> String {
    println!("Quelle case voulez-vous viser ?");

    let mut coup = String::new();

    io::stdin().read_line(&mut coup).expect("Échec de l'entrée");

    coup.trim().to_string()
}

pub(crate) fn coup(a: &str) -> (usize, usize){
    let mut colonne = 0;
    let mut ligne  = 0;

    let coup = Vec::from(a);

    let c = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    let l = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

    if coup.len() == 3 {
        for character in a.chars() {
            for i in 0..10 {
                if character.to_string() == c[i] {
                    colonne = i;
                    break;
                }
            }
        }
        ligne = 9;
    } else {
        for character in a.chars() {
            for i in 0..10 {
                if character.to_string() == c[i] {
                    colonne = i;
                    break;
                }
                else{
                    println!("Coup invalide");
                }
            }

            for j in 0..10 {
                if character.to_string() == l[j] {
                    ligne = j;
                    break;
                }
                else{
                    println!("Coup invalide !")
                }
            }
        }
    }

    (ligne, colonne)
}