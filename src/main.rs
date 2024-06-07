use std::fs;
use std::fs::File;
use std::time::Instant;

use rand::random;

fn main() {
    let start = Instant::now();

    // Créer un dossier spécifique
    let dir = "dossier_specifique";
    fs::create_dir_all(dir).unwrap();

    for _ in 0..50000 {
        let random_number: u32 = random();
        let file_name = format!("{}/fichier_{}", dir, random_number);
        File::create(file_name).unwrap();
    }

    // Supprimer tous les fichiers dans le dossier spécifique
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        fs::remove_file(entry.path()).unwrap();
    }

    let duration = start.elapsed();
    println!("Temps d'exécution: {:?}", duration);
}