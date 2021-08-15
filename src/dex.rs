use std::io;
use colored::*;

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub id: u32,
    pub main_type: String, 
}

pub struct Pokedex {
  pub list: Vec<Pokemon>
}

#[allow(dead_code)]
impl Pokedex {
  pub fn new() -> Pokedex {
    Pokedex { list: [].to_vec() }
  }

  pub fn add_to_pokedex(&mut self) {
    let mut pokemon_name = String::new();
    let mut pokemon_id = String::new();
    let mut pokemon_type = String::new();

    println!("Type the pokemon name: ");
    io::stdin().read_line(&mut pokemon_name).expect("Failed to read message");
    let pokemon_name = pokemon_name.trim();

    println!("Type the pokemon id: ");
    io::stdin().read_line(&mut pokemon_id).expect("Failed to read message");
    let pokemon_id = match pokemon_id.trim().parse() {
      Ok(num) => num,
      Err(_) => 0
    };

    println!("Type the pokemon main type: ");
    io::stdin().read_line(&mut pokemon_type).expect("Failed to read message");
    let pokemon_type = pokemon_type.trim();

    self.list.push(Pokemon {
      name: pokemon_name.to_string(),
      id: pokemon_id,
      main_type: pokemon_type.to_string()
    });

    println!();
  }

  pub fn remove_pokemon(&mut self) {
    println!("Remove a pokemon");

    let mut pokemon_id = String::new();
    println!("Type the pokemon id to remove: ");
    io::stdin().read_line(&mut pokemon_id).expect("Failed to read message");
    let pokemon_id = match pokemon_id.trim().parse() {
      Ok(num) => num,
      Err(_) => 0
    };

    self.list.retain(|x| x.id != pokemon_id);
    println!();

  }

  pub fn print_pokedex(&self) {

    for pokemon in self.list.iter() {
      println!("#################");
      println!("Name: {}", pokemon.name.green());
      println!("ID: {}", pokemon.id.to_string().green());
      println!("Main type: {}", pokemon.main_type.green());
      println!("#################");
      println!();
    }

  }
}
