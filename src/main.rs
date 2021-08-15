use std::io;
mod dex;
use crate::dex::*;

fn main() {
    println!();
    println!("###### POKEDEX ######");
    println!();

    let mut pokedex = Pokedex::new();

    loop {
      println!("What you wanna do? \n1. Add Pokemon\n2. View all pokemons\n3. Remove Pokemon\n4. Quit");

      let mut choice = String::new();
      io::stdin().read_line(&mut choice).expect("Failed to read message");
      let choice = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 3
      };

      println!();

      if choice == 1 {
        pokedex.add_to_pokedex();
      } else if choice == 2 {
        pokedex.print_pokedex();
      } else if choice == 3 {
        pokedex.remove_pokemon();
      } else if choice == 4 {
        break;
      } else {
        break;
      }
    }
}
