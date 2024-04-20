// main.rs

mod paises;

use paises::{Pais, read_paises_from_file, write_paises_to_file};
use std::io;

fn main() -> io::Result<()> {
    let filename = "data/paises.csv";
    let mut paises = read_paises_from_file(filename)?;

    // Exemplo de uso
    for pais in &paises {
        println!("{:?}", pais);
    }

    // Adicionar um novo país
    let novo_pais = Pais::new("Brasil".to_string(), "América do Sul".to_string());
    paises.push(novo_pais);

    // Marcar um país como visitado
    if let Some(pais) = paises.iter_mut().find(|p| p.nome == "Brasil") {
        pais.visitado = paises::VisitStatus::Visited;
    }

    // Marcar um país como favorito
    if let Some(pais) = paises.iter_mut().find(|p| p.nome == "Brasil") {
        pais.favorito = true;
    }

    write_paises_to_file(filename, &paises)?;

    Ok(())
}
