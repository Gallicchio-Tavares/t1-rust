use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] 
pub enum VisitStatus {
    SemViagem,
    QueroVisitar,
    ViagemMarcada,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pais {
    pub nome: String,
    pub continente: String,
    pub visitado: VisitStatus,
    pub favorito: bool,
    pub jafui: u32,
}

impl Pais {
    pub fn new(nome: String, continente: String) -> Self {
        Pais {
            nome,
            continente,
            visitado: VisitStatus::SemViagem,
            favorito: false,
            jafui: 0,
        }
    }

    pub fn from_str(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 5 {
            let nome = parts[0].trim().to_string();
            let continente = parts[1].trim().to_string();
            let visitado = match parts[2].trim() {
                "ViagemMarcada" => VisitStatus::ViagemMarcada,
                "QueroVisitar" => VisitStatus::QueroVisitar,
                _ => VisitStatus::SemViagem,
            };
            let favorito = match parts[3].trim() {
                "true" => true,
                _ => false,
            };
            let jafui = parts[4].trim().parse().unwrap_or(0); // Converta para u32

            Some(Pais {
                nome,
                continente,
                visitado,
                favorito,
                jafui,
            })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{},{}", self.nome, self.continente, self.visitado_str(), self.favorito, self.jafui)
    }

    pub fn visitado_str(&self) -> &'static str {
        match self.visitado {
            VisitStatus::ViagemMarcada => "ViagemMarcada",
            VisitStatus::SemViagem => "SemViagem",
            VisitStatus::QueroVisitar => "QueroVisitar",
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
