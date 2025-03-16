# Portfolio

Ici se trouve tout mes projets de programmation. Pour lancer les projets écrit en rust, il faut installer le gestionnaire de paquet de Rust (cargo) et lancer la commande suivant depuis le fichier du projet:
```
cargo run
```

# Principaux Projets

## Rust-math

Ce projet avait pour but de m'entrainer à utiliser les generics en créant un programme capable de représenter les suite arithmetiques et géometriques et de calculer n'importe quelle valeur de la suite au range $n$.

J'ai essayer de créer un macro qui me permettrerait de générer toutes les fonctions de tests mais ça va me prendre beaucoup plus de temps que je pense, les tests vont probablement être intégrer grâce à la macro en août-septembre.

### Example de création d'une suite

```rs
use rust-math::*;

/// SequenceVariant est le type qui va définir comment les calcul vont être effectué: 
/// - SequenceVariant::Explicit => calcule directe du terme demandé par un formule explicite
/// - SequenceVariant::Recurence => calcule par récurence du terme demandé par
///   la formule fourni dans l'un des constructeurs de SequenceType
/// 
/// SequenceType est le type qui va définir si la sequence est arithmétique ou géometrique
///  - SequenceType::Arithmetic => séquence arithmétique
///  - SequenceType::Geometric => séquence géometrique

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

### Note

La macro du projet qui génère des tests deviendra très probablement une librairie pour rust.

## Screaming_hunter

Le but de ce projet était de réecrire le jeux https://mystoxi.itch.io/screaming-hunter fait par un moi et mes amis de GodoScript à Rust en utilisant le moteur de jeux Bevy. Le deuxième objectif était de pouvoir faire apparaitre plus d'entités (les vagues rouges simbolisant le cris de la chauve-souris) sans faire laguer l'ordinateur de l'utilisateur en utilisant les performances de Rust.

### Petite explication du jeux

Le joueur joue une chauve souris qui doit attraper et manger une souris cacher dan le noir. Pour la trouver, la chauve-souris doit poursser un cris (barre espace). Ce cris est envoyer dans toute les directions et rebondi sur les obstacles (comme par example la souris) et revient vers la chauve souris. La chauve-souris peut ainsi aller sur la souris et la manger, Victoire !

### Écran d'accueil du jeux

![Capture d'écran du jeux Screaming Hunter](capture-decran-screaming-hunter.png)

## Snake en python

Ce projet ma permit d'apprendre comment utiliser la librairie pygame pour faire des jeux en python.

### Note

Pour jouer au jeux, lancer le programme dans le terminal depuis le fichier avec la commande suivante:
```
python3 snake.py
```

### Image du jeux

![Capture d'écran du jeux snake écrit en python](Snake-python-gameplay.png)

## App-serre-ui

C'est un projet que j'ai commencé au lycée en spécialité SI. Le but est de créer une serre automatisé qui envoie sur une application sur smartphone les données récupérées par les capteurs de la serre (température, humidité, luminosité, ...). Grace à l'application, l'utilisateur peut passer la serre en mode manuel afin de pouvoir controler tout les moteur de la serre par lui-même.

L'objectif était de faire communiqué l'application écrite en Rust avec la carte arduino codé en Arduino.

## Exercice MP2I

C'est un exercice sur l'algorithme de quine que j'ai commencé un peux à la MP2I de Vannes avec des élèves de l'école pendant une journée d'intégration. Ils le faisaient en OCaml et je me suis dit que se serait une bonne idée de le faire un rust pour m'entrainer sur la récursivité. Le pdf de l'exercice et compris dans le fichier du projet.

## Rock Paper Scissor 

Ce projet avait pour but de recréer le jeux pierre feuille sciseau dans le terminale. Les joueurs choisissent leur pseudo et commmence la partie en choisissant pierre, papier ou sciseau de manière caché, cela permet d'éviter toutes triches (sauf si le joueur adverse regarde le choix que l'on tape sur le clavier). Un compteur se met alors en route et au bout de 3 secondes, le pseudo du gagnant est affiché ainsi que son choix (pierre, papier ou sciseau)
