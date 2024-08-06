use std::collections::HashSet;
use rand::Rng;
use std::io;

fn main() {
    let palavras = vec!["banana", "abacate", "laranja", "maçã"];
    let palavra_secreta = palavras[rand::thread_rng().gen_range(0..palavras.len())];

    let mut letras_acertadas: HashSet<char> = HashSet::new();
    let mut tentativas = 0;

    loop {
        println!("Adivinhe a palavra:");

        let mut chute = String::new();
        io::stdin()
            .read_line(&mut chute)
            .expect("Falha ao ler a linha");

        let chute = chute.trim().chars().next().unwrap();

        if letras_acertadas.contains(&chute) {
            println!("Você já chutou essa letra!");
            continue;
        }

        letras_acertadas.insert(chute);
        tentativas += 1;

        let mut acertos = 0;
        for letra in palavra_secreta.chars() {
            if letras_acertadas.contains(&letra) {
                print!("{}", letra);
                acertos += 1;
            } else {
                print!("_");
            }
        }
        println!();

        if acertos == palavra_secreta.len() {
            println!("Parabéns! Você acertou a palavra em {} tentativas.", tentativas);
            break;
        }
    }
}
