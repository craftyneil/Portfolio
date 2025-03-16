# Portfolio

Ici se trouve tout mes projets de programmation 

# Pricipaux Projets

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
// U(n) = 1 * 3^n et n = 3 donc U(3) = 1 * 3^3 = 27
```

#### Note

La macro du projet qui génère des tests deviendra très probablement une librairie pour rust.

## Screaming_hunter

Le but de ce projet était de réecrire le jeux https://mystoxi.itch.io/screaming-hunter fait par un moi et mes amis de GodoScript à Rust en utilisant le moteur de jeux Bevy. Le deuxième objectif était de pouvoir faire apparaitre plus d'entités (les vagues rouges simbolisant le cris de la chauve-souris) sans faire laguer l'ordinateur de l'utilisateur en utilisant les performances de Rust.

### Petite explication du jeux

Le joueur joue une chauve souris qui doit attraper et manger une souris cacher dan le noir. Pour la trouver, la chauve-souris doit poursser un cris (barre espace). Ce cris est envoyer dans toute les directions et rebondi sur les obstacles (comme par example la souris) et revient vers la chauve souris. La chauve-souris peut ainsi aller sur la souris et la manger, Victoire !

### Écran d'accueil du jeux

![Capture d'écran du jeux Screaming Hunter](capture-decran-screaming-hunter.png)

## Snake en python

Ce projet ma permit d'apprendre comment utiliser la librairie pygame pour faire des jeux en python

### Image du jeux

![Capture d'écran du jeux snake écrit en python](Snake-python-gameplay.png)