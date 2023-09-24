use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Ouvrir ou créer un fichier en écriture (créera le fichier s'il n'existe pas)
    let mut file = File::create("votre_fichier.txt")?;

    // Écrire des données dans le fichier
    file.write_all(b"Ceci est un exemple de texte.")?;

    println!("Fichier créé ou ouvert avec succès.");

    Ok(())
}
