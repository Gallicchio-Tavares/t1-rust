use crate::paises::{Pais, VisitStatus};
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::File;
use csv::ReaderBuilder;
use colored::*;
//extern crate inflector;
use inflector::Inflector;

const DADOS_FILE: &str = "data/dados.csv";

pub fn adicionar_pais(paises: &mut Vec<Pais>) -> io::Result<()> {
    print!("Digite o nome do país que deseja adicionar: ");
    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin().read_line(&mut nome)?;

    let nome_formatado = nome.trim().to_title_case();

    // Verifica se o país já existe na lista
    if paises.iter().any(|pais| pais.nome.trim() == nome_formatado) {
        println!("\n{}","Este país já foi adicionado!".red());
        return Ok(());
    }

    let continente = obter_continente(&nome_formatado)?;
    let novo_pais = Pais::new(nome_formatado, continente);
    paises.push(novo_pais);

    Ok(())
}


fn obter_continente(nome_pais: &str) -> io::Result<String> {
    let file = File::open(DADOS_FILE)?;
    let mut reader = ReaderBuilder::new().flexible(true).trim(csv::Trim::All).from_reader(file);

    for result in reader.records() {
        let record = result?;
        if record.get(0) == Some(nome_pais) {
            return Ok(record.get(1).unwrap().to_string());
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "País não encontrado"))
}

pub fn remover_pais(paises: &mut Vec<Pais>) {
    print!("Digite o nome do país que deseja remover: ");
    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();

    let nome_formatado = nome.trim().to_title_case();

    if let Some(index) = paises.iter().position(|pais| pais.nome.trim() == nome_formatado) {
        paises.remove(index);
        println!("País removido com sucesso.");
    } else {
        println!("País não encontrado.");
    }
}

impl Pais {

    pub fn atualizar_visitas(&mut self, visitas: u32) {
        self.jafui = visitas;
    }

    pub fn definir_status_viagem(&mut self, status: VisitStatus) {
        self.visitado = status;
    }
}

fn definir_status_viagem(pais: &mut Pais) {
    println!("Escolha o novo status de viagem:");
    println!("1. Sem Viagem");
    println!("2. Quero Visitar");
    println!("3. Viagem Marcada");

    print!("Escolha uma opção: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<u32>() {
        Ok(choice) => match choice {
            1 => pais.definir_status_viagem(VisitStatus::SemViagem),
            2 => pais.definir_status_viagem(VisitStatus::QueroVisitar),
            3 => pais.definir_status_viagem(VisitStatus::ViagemMarcada),
            _ => println!("Opção inválida"),
        },
        Err(_) => println!("Opção inválida"),
    }
}

fn marcar_como_favorito(pais: &mut Pais) {
    pais.favorito = true;
    println!("\n{} marcado como favorito", pais.nome.yellow());
}

fn remover_favorito(pais: &mut Pais) {
    pais.favorito = false;
    println!("\n{} removido como favorito", pais.nome.yellow())
}

fn marcar_quantas_vezes_foi(pais: &mut Pais) {

    print!("Digite o número de vezes que você já visitou {}: ", pais.nome);
    io::stdout().flush().unwrap();
    let mut visitas = String::new();
    io::stdin().read_line(&mut visitas).unwrap();

    match visitas.trim().parse::<u32>() {
        Ok(visitas) => {
            pais.atualizar_visitas(visitas);
            println!("Número de visitas atualizado com sucesso.");
        }
        Err(_) => println!("Número de visitas inválido."),
    }
}

pub fn definir_status_pais(paises: &mut Vec<Pais>){
    print!("Digite o nome do país que deseja modificar: ");
    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).unwrap();

    let nome_formatado = nome.trim().to_title_case();

    if let Some(pais) = paises.iter_mut().find(|p| p.nome.trim() == nome_formatado) {
        println!("Escolha a ação:");
        println!("1. Marcar como favorito");
        println!("2. Remover como favorito");
        println!("3. Definir status de viagem");
        println!("4. Marcar quantas vezes visitou");

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => marcar_como_favorito(pais),
                2 => remover_favorito(pais),
                3 => definir_status_viagem(pais),
                4 => marcar_quantas_vezes_foi(pais),
                _ => println!("Opção inválida"),
            },
            Err(_) => println!("Opção inválida"),
        }
    } else {
        println!("País não encontrado.");
    }
}

pub fn exibir_paises(paises: &Vec<Pais>, mostrar_favoritos: bool) {
    let mut paises_por_continente: HashMap<String, Vec<&Pais>> = HashMap::new();

    // Agrupar os países por continente
    for pais in paises {
        paises_por_continente
            .entry(pais.continente.clone())
            .or_insert_with(Vec::new)
            .push(pais);
    }

    // Imprimir os países agrupados por continente
    for (continente, paises) in &paises_por_continente {
        println!("\n=== {} ===", continente.purple());
        for pais in paises {
            if !mostrar_favoritos || pais.favorito {
                let status_viagem = match pais.visitado {
                    VisitStatus::QueroVisitar => "quero visitar",
                    VisitStatus::ViagemMarcada => "tenho uma viagem marcada",
                    _ => "não penso em visitar no momento"
                };
                println!(
                    "{} - visitei {} vez(es) e {}",
                    pais.nome.yellow(), 
                    pais.jafui.to_string().blue(),
                    status_viagem
                );
                
            }
        }
    }
}
