// Importe la librairie io pour input / output | et tout les autre librairies
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // Affiche des phrases à l'écran
    println!("Guess the number !");

    // Crée le nombre aléatoire
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    println!("Please input your guess.");

    loop {
        // Crée un endroit où stoker la variriable de l'input
        // mut permet à ce que la variable soit mutable donc changeable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // read_line prends n'importe quelle input de l'utisateur
            .expect("Failed to read line"); // sert à envoyer qqc si l'input n'est pas celle voulu
    
        // Oblige l'utilisateur à mettre un entier et pas des lettres, permet aussi d'éviter l'erreur dù à la ligne 18
        let guess: u32 = guess.trim().parse().expect("Please type a number !");

        // Affiche l'input de l'utilisateur
        // Pour afficher des variables dans des phrases il faut mettre des {} dans la phrase -> https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#printing-values-with-println-placeholders
        // println!("You guessed : {}", guess);

        // Compare le guess avec le chiffre secret
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("GG! You win");
                break;
            }
        }
    }

}
