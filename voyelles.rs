// Exercice : Trouver le nombre de voyelles dans une chaîne de caractères
//
// Objectif : Écrire une fonction qui prend en entrée une chaîne de caractères 
// et retourne le nombre de voyelles qu'elle contient.
//
// Les voyelles considérées sont : 'a', 'e', 'i', 'o', 'u', et leur équivalent en majuscule.
//
// Exemple :
// - "hello" -> 2
// - "Rust programming" -> 5

fn count_vowels(s: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    s.chars()
        .filter(|c| vowels.contains(&c.to_ascii_lowercase())) // Vérifie si c'est une voyelle
        .count()
}

fn main() {
    let text = "Rust programming";
    let count = count_vowels(text);
    println!("Le nombre de voyelles dans '{}' est : {}", text, count);
}
