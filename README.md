# Portfolio

Ici se trouve mes projets les plus importants

## Rust-math

Ce projet avait pour but de m'entrainer à utiliser les generics en créant un programme capable de représenter les suite arithmetiques et géometriques et de calculer n'importe quelle valeur de la suite au range $n$.

J'ai essayer de créer un macro qui me permettrerait de générer toutes les fonctions de tests mais ça va me prendre beaucoup plus de temps que je pense, les tests vont probablement être intégrer grâce à la macro en août-septembre.

### Example de création d'une suite

```rs
use rust-math::*;

let my_sequence = SequenceVariant::Explicit {
    initial_term: 1,
    formula: SequenceType::from_fn(|n| 3 * n)
}

assert_eq!(my_sequence.initial_term(), 1);

assert_eq!(my_sequence.reason(), 3);


// J'ai fait en sorte que l'utilisateur de puisse pas calculer le terme n 
// avec n un nombre négatif car n doit être un entier naturel
assert_eq!(my_sequence.nth_term(3), 27);
// U(n) = 1 * 3^n et n = 3 donc U(3) = 1 *3^3 = 27
```