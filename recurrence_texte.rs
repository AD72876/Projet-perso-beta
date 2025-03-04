// Exercice : Compter le nombre d'occurrences de chaque mot dans une phrase
//
// Objectif : Écrire une fonction qui prend une phrase en entrée et retourne 
// un dictionnaire contenant le nombre d'occurrences de chaque mot.
//
// Les mots doivent être comparés en ignorant la casse et la ponctuation.
//
// Exemple :
// - "Rust est génial, Rust est rapide !" 
//   -> {"rust": 2, "est": 2, "génial": 1, "rapide": 1}

use std::collections::HashMap;

fn word_count(phrase: &str) -> HashMap<String, usize> {
    let mut occurrences = HashMap::new();

    // Transformer la phrase en minuscules et filtrer la ponctuation
    let cleaned_phrase = phrase
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace()) // Garde uniquement les lettres et espaces
        .collect::<String>();

    // Séparer la phrase en mots et compter les occurrences
    for word in cleaned_phrase.split_whitespace() {
        *occurrences.entry(word.to_string()).or_insert(0) += 1;
    }

    occurrences
}

fn main() {
    let phrase = "Rust est génial, Rust est rapide !";
    let word_counts = word_count(phrase);

    println!("Occurrences des mots :");
    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
}
