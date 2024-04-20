mod paises;
mod funcoes;
mod fato_random;

use paises::{read_paises_from_file, write_paises_to_file};
use funcoes::{adicionar_pais, remover_pais, exibir_paises, definir_status_pais};
use fato_random::ver_fato_aleatorio_de_pais;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "data/paises.csv";
    let mut paises = read_paises_from_file(filename)?;
    println!("Boas vindas ao seu gerenciador de viagens!");

    loop {
        println!("\n=== Menu ===");
        println!("1. Adicionar país a lista"); // create
        println!("2. Remover país de lista"); // delete
        println!("3. Exibir lista de países"); // read
        println!("4. Modificar status de um país"); // update -> adicionar favorito, marcar se já foi, modificar VisitStatus
        println!("5. Sair");

        print!("Escolha uma opção: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => adicionar_pais(&mut paises)?,
                2 => remover_pais(&mut paises),
                3 => {
                    let mostrar_favoritos = obter_mostrar_favoritos()?;
                    exibir_paises(&paises, mostrar_favoritos);
                },
                4 => definir_status_pais(&mut paises),
                5 => {
                    println!("Até mais! Aqui seu fato aleatório do dia:");
                    match ver_fato_aleatorio_de_pais() {
                        Ok(paragrafo) => println!("\n{}", paragrafo),
                        Err(e) => eprintln!("Erro: {}", e),
                    }
                    write_paises_to_file(filename, &paises)?;
                    break;
                }
                _ => println!("Opção inválida"),
            },
            Err(_) => println!("Opção inválida"),
        }
    }

    Ok(())
}

fn obter_mostrar_favoritos() -> io::Result<bool> {
    loop {
        print!("Deseja ver apenas os países favoritos? (S/N): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "s" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("Resposta inválida. Por favor, responda com S ou N."),
        }
    }
}