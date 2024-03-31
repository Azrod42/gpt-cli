use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
};

use serde::{de::DeserializeOwned, Serialize};

pub fn store_tmp_file<T: Serialize>(payload: &T, filename: &str) {
    let path: String = std::env::temp_dir().to_string_lossy().to_string() + filename;

    let mut file: File = match OpenOptions::new().write(true).create(true).open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("Impossible d'ouvrir le fichier. Vérifiez le chemin et les autorisations.");
            return;
        }
    };

    if let Ok(metadata) = file.metadata() {
        let mut bytes: Vec<u8> = Vec::new();
        serde_json::to_writer(&mut bytes, &payload).unwrap();
        if metadata.len() == 0 {
            if let Err(_) = file.write_all(&bytes) {
                println!("Erreur lors de l'écriture dans le fichier.")
            }
        }
    }
}

pub fn read_tmp_file<T: DeserializeOwned>(filename: &str) -> Option<T> {
    let path: String = std::env::temp_dir().to_string_lossy().to_string() + filename;

    let file = File::open(&path);
    let file = match file {
        Ok(file) => file,
        Err(_) => return None,
    };
    let reader = BufReader::new(file);
    let data = serde_json::from_reader::<_, T>(reader);
    match data {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub fn remove_tmp_file(filename: &str) {
    let path: String = std::env::temp_dir().to_string_lossy().to_string() + filename;

    let _ = std::fs::remove_file(path);
}
