// Exercice : Vérification d'un palindrome
//
// Un palindrome est un mot, une phrase ou une séquence de caractères qui se lit de la même manière
// de gauche à droite et de droite à gauche (par exemple, "radar", "level", etc.).
//
// Objectif : Implémenter une fonction qui vérifie si un mot donné est un palindrome. La fonction doit
// ignorer les espaces, la ponctuation et la casse des lettres. La fonction retournera `true` si le mot
// est un palindrome, et `false` sinon.

fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric()) // On garde uniquement les caractères alphanumériques
        .collect();

    let reversed: String = cleaned.chars().rev().collect(); // On inverse la chaîne

    cleaned.eq_ignore_ascii_case(&reversed) // On compare les deux chaînes en ignorant la casse
}

fn main() {
    let word = "Radar"; // Test avec un palindrome
    if is_palindrome(word) {
        println!("'{}' est un palindrome!", word);
    } else {
        println!("'{}' n'est pas un palindrome.", word);
    }
}
