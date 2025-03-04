// Exercice : Trouver la longueur du plus long sous-tableau de nombres distincts
//
// Objectif : Écrire une fonction qui prend un tableau d'entiers en entrée et retourne 
// la longueur du plus long sous-tableau contenant uniquement des éléments distincts.
//
// Exemple :
// - [4, 2, 1, 3, 2, 1, 5, 6, 2] -> 5 (le plus long sous-tableau sans répétition est [3, 2, 1, 5, 6])
// - [1, 2, 3, 4, 5] -> 5 (tout le tableau est distinct)
// - [5, 5, 5, 5] -> 1 (seul un élément peut être pris sans répétition)

use std::collections::HashSet;

fn longest_unique_subarray(arr: &[i32]) -> usize {
    let mut seen = HashSet::new(); // Ensemble pour suivre les éléments uniques
    let mut left = 0; // Pointeur gauche
    let mut max_length = 0;

    for right in 0..arr.len() {
        while seen.contains(&arr[right]) {
            seen.remove(&arr[left]); // Retirer les éléments jusqu'à éliminer le doublon
            left += 1;
        }

        seen.insert(arr[right]); // Ajouter le nouvel élément unique
        max_length = max_length.max(right - left + 1);
    }

    max_length
}

fn main() {
    let numbers = [4, 2, 1, 3, 2, 1, 5, 6, 2];
    let result = longest_unique_subarray(&numbers);
    println!("Longueur du plus long sous-tableau distinct : {}", result);
}
