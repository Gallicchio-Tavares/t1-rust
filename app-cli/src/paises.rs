//use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
pub enum VisitStatus {
    NotVisited,
    Visited,
}

#[derive(Debug)]
pub struct Pais {
    pub nome: String,
    pub continente: String,
    pub visitado: VisitStatus,
    pub favorito: bool,
}

impl Pais {
    pub fn new(nome: String, continente: String) -> Self {
        Pais {
            nome,
            continente,
            visitado: VisitStatus::NotVisited,
            favorito: false,
        }
    }

    pub fn from_str(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 4 {
            let nome = parts[0].trim().to_string();
            let continente = parts[1].trim().to_string();
            let visitado = match parts[2].trim() {
                "Visited" => VisitStatus::Visited,
                _ => VisitStatus::NotVisited,
            };
            let favorito = match parts[3].trim() {
                "true" => true,
                _ => false,
            };
            Some(Pais {
                nome,
                continente,
                visitado,
                favorito,
            })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.nome, self.continente, self.visitado_str(), self.favorito)
    }

    pub fn visitado_str(&self) -> &'static str {
        match self.visitado {
            VisitStatus::Visited => "Visited",
            VisitStatus::NotVisited => "NotVisited",
        }
    }
}

pub fn read_paises_from_file(filename: &str) -> io::Result<Vec<Pais>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut paises = Vec::new();

    for line in reader.lines() {
        if let Some(pais) = Pais::from_str(&line?) {
            paises.push(pais);
        }
    }

    Ok(paises)
}

pub fn write_paises_to_file(filename: &str, paises: &[Pais]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename)?;

    for pais in paises {
        writeln!(file, "{}", pais.to_string())?;
    }

    Ok(())
}
