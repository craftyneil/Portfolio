use round::round_down;

fn main() {
    println!("Hello, world!");
}

fn division2(mut dec: u32) -> Vec<u32> {
    let mut resultat: Vec<u32> = vec![];
    let mut reste: u32;

    while round_down(dec as f64, 0) != 0.0 && dec > 0 {
        reste = dec % 2;
        dec = round_down(dec as f64 / 2.0, 0) as u32;
        resultat.push(reste);
    }
    if dec / 2 == 1  || dec == 1 {
        resultat.push(dec % 2)
    }
    resultat
}
fn affichage<T>(mut liste: Vec<T>) -> Vec<T> {
    liste.reverse();
    liste
}
fn equivalence_hexa(val: u32) -> String {
    match val {
        10 => "A".to_string(),
        11 => "B".to_string(),
        12 => "C".to_string(),
        13 => "D".to_string(),
        14 => "E".to_string(),
        15 => "F".to_string(),
        _ => val.to_string()
    }
} 

fn division16(mut n: u32) -> Vec<String> {
    let mut resultat: Vec<String> = vec![];
    let mut reste: u32;

    while round_down(n as f64, 0) != 0.0 && n > 0 {
        reste = n % 16;
        n = round_down(n as f64 / 16.0, 0) as u32;
        resultat.push(equivalence_hexa(reste));
    }
    if n / 16 == 1  || n == 1 {
        resultat.push(equivalence_hexa(n % 16))
    }
    resultat
}

fn liste(n: String) -> Vec<String> {
    let mut resultat: Vec<String> = vec![];

    for i in 0..n.len() {
        resultat.push(n.chars().nth(i).unwrap().to_string());
    }
    resultat
}

fn equivalence_dec(n: &str) -> u32 {
    match n {
        "A" => 10,
        "B" => 11,
        "C" => 12,
        "D" => 13,
        "E" => 14,
        "F" => 15,
        _ => n.parse::<u32>().unwrap()
    }
}

fn hex_dec(n: String) -> u32 {
    let mut resultat: u32 = 0;

    for i in 0..n.len() {
        resultat = resultat + equivalence_dec(&n.chars().nth(i).unwrap().to_string()) * u32::pow(16, n.len() as u32 - i as u32 - 1 );
    }
    resultat
}

fn dec_bin(n: u32)  -> Vec<u32> {
    affichage(division2(n))
}

fn dec_hex(n: u32) -> Vec<String> {
    affichage(division16(n))
}

fn bin_dec(n: String) -> u32 {
    let mut resultat: u32 = 0;
    for i in 0..n.len() {
        resultat = resultat + equivalence_dec(&n.chars().nth(i).unwrap().to_string()) * u32::pow(2, n.len() as u32 - i as u32 - 1 );
    }
    resultat
}

fn hex_bin(n: String) -> Vec<u32> {
    affichage(division2(hex_dec(n)))
}

fn bin_hex(n: String) -> Vec<String> {
    n.chars()
        .collect::<Vec<char>>() // Convertir la chaîne en vecteur de caractères
        .chunks(4)               // Diviser le vecteur en morceaux de 4 bits
        .map(|chunk| chunk.iter().collect::<String>()) // Convertir chaque morceau en String
        .map(|character| equivalence_hexa(bin_dec(character))) // Convertir chaque morceau en hexadécimal
        .collect::<Vec<String>>() // Collecter les résultats dans un vecteur

}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_division2() {
        let input: u32 = 10;
        let expected_output: Vec<u32> = vec![0, 1, 0, 1];

        let output: Vec<u32> = division2(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_affichage() {
        let input: Vec<i32> = vec![0, 0, 1, 1];
        let expected_output: Vec<i32> = vec![1, 1, 0, 0];

        let output: Vec<i32> = affichage(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test2_affichage() {
        let input: Vec<i32> = vec![0, 1, 0, 1];
        let expected_output: Vec<i32> = vec![1, 0, 1, 0];

        let output: Vec<i32> = affichage(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_equivalence_hexa() {
        let input: u32 = 0;
        let expected_output: String = "0".to_string();

        let output: String = equivalence_hexa(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test2_equivalence_hexa() {
        let input: u32 = 10;
        let expected_output: String = "A".to_string();

        let output: String = equivalence_hexa(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_division16() {
        let input: u32 = 1;
        let expected_output: Vec<String> = vec!["1".to_string()];

        let output: Vec<String> = division16(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test2_division16() {
        let input: u32 = 689;
        let expected_output: Vec<String> = vec!["1".to_string(), "B".to_string(), "2".to_string()];

        let output: Vec<String> = division16(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_liste() {
        let input: String = "126".to_string();
        let expected_output: Vec<String> = vec!["1".to_string(), "2".to_string(), "6".to_string()];

        let output: Vec<String> = liste(input);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test2_liste() {
        let input: String = "2F16".to_string();
        let expected_output: Vec<String> = vec!["2".to_string(), "F".to_string(), "1".to_string(), "6".to_string()];

        let output: Vec<String> = liste(input);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_equivalence_dec() {
        let input: &str = "1";
        let expected_output: u32 = 1;

        let output: u32 = equivalence_dec(input);
        assert_eq!(output, expected_output);
    }
    
    #[test]
    fn test2_equivalence_dec() {
        let input: &str = "A";
        let expected_output: u32 = 10;

        let output: u32 = equivalence_dec(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_hex_dec() {
        let input: String = "8".to_string();
        let expected_output: u32 = 8;

        let output: u32 = hex_dec(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test2_hex_dec() {
        let input: String = "2B1".to_string();
        let expected_output: u32 = 689;

        let output: u32 = hex_dec(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_dec_bin() {
        let input: u32 = 10;
        let expected_output: Vec<u32> = vec![1, 0, 1, 0];

        let output: Vec<u32> = dec_bin(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_dec_hex() {
        let input: u32 = 689;
        let expected_output: Vec<String> = vec!["2".to_string(), "B".to_string(), "1".to_string()];

        let output: Vec<String> = dec_hex(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_bin_dec() {
        let input: String = "1010".to_string();
        let expected_output: u32 = 10;

        let output: u32 = bin_dec(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test2_bin_dec() {
        let input: String = "10".to_string();
        let expected_output: u32 = 2;

        let output: u32 = bin_dec(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_hex_bin() {
        let input: String = "A".to_string();
        let expected_output: Vec<u32> = vec![1, 0, 1, 0];

        let output: Vec<u32> = hex_bin(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_bin_hex() {
        let input: String = "1010".to_string();
        let expected_output: Vec<String> = vec!["A".to_string()];

        let output: Vec<String> = bin_hex(input);
        assert_eq!(output, expected_output);
    }
}