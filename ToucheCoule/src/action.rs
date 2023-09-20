use std::io;

fn futurcoup() -> String {
    println!("Quelle case voulez-vous viser ?");

    let mut coup = String::new();

    io::stdin()
        .read_line(&mut coup)
        .expect("Échec de l'entrée");

    println!("Votre coup est : {}", coup);

    coup.trim().to_string()
}

fn coup(a: &str) {
    let mut colonne = 0;
    let mut ligne: i32 = 0;
    let c = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    let l = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

    for character in a.chars() {
        for i in 0..10 {
            if character.to_string() == c[i] {
                colonne = i;
                break;
            }
        }

        for j in 0..10 {
            if character.to_string() == l[j] {
                ligne = j as i32;
                break;
            }
        }
    }

    println!("La colonne est {}, et la ligne est {}", colonne, ligne);
}

fn main() {
    let a = futurcoup();
    coup(&a);
}
