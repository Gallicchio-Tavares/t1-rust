#[cfg(test)]
mod tests {
    use crate::{fato_random::{obter_paragrafo, ver_fato_aleatorio_de_pais}, funcoes::adicionar_pais, paises::write_paises_to_file};
    use std::fs;
    use tempfile::NamedTempFile;
    use crate::paises::{self, read_paises_from_file, Pais};
    use std::io::{self, Write};

    #[test]
    fn test_read_paises_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Brasil,América do Sul,ViagemMarcada,false,2").unwrap();
        writeln!(temp_file, "Argentina,América do Sul,SemViagem,true,0").unwrap();
        writeln!(temp_file, "Itália,Europa,QueroVisitar,true,1").unwrap();

        let file_path = temp_file.into_temp_path();

        let result = read_paises_from_file(file_path.to_str().unwrap());

        assert!(result.is_ok());
        let paises = result.unwrap();
        assert_eq!(paises.len(), 3);

        assert_eq!(paises[0].nome, "Brasil");
        assert_eq!(paises[0].continente, "América do Sul");
        assert_eq!(paises[0].visitado, paises::VisitStatus::ViagemMarcada);
        assert_eq!(paises[0].favorito, false);
        assert_eq!(paises[0].jafui, 2);
    }

    #[test]
    fn test_write_paises_to_file() -> io::Result<()> {

        let filename = "test_paises.csv";

        let paises = vec![
            Pais::new("Brasil".to_string(), "América do Sul".to_string()),
            Pais::new("Canadá".to_string(), "América do Norte".to_string()),
            Pais::new("Espanha".to_string(), "Europa".to_string()),
        ];

        write_paises_to_file(filename, &paises)?;
        assert!(fs::metadata(filename).is_ok());
        let paises_lidos = read_paises_from_file(filename)?;
        assert_eq!(paises_lidos, paises);
        fs::remove_file(filename)?;

        Ok(())
    }

    #[test]
    fn test_ver_fato_aleatorio_de_pais() {
        println!("Iniciando teste test_ver_fato_aleatorio_de_pais...");

        match ver_fato_aleatorio_de_pais() {
            Ok((pais, _)) => {
                assert!(!pais.is_empty());
                println!("Nome do país obtido: {}", pais);
            }
            Err(_) => panic!("Erro ao obter fato aleatório de país"),
        }
        println!("Teste test_ver_fato_aleatorio_de_pais concluído!");
    }

    #[test]
    fn test_obter_paragrafo() {
        let url = "https://pt.wikipedia.org/wiki/Senegal";

        match obter_paragrafo(url) {
            Ok(paragrafo) => {
                assert!(!paragrafo.is_empty(), "O parágrafo obtido está vazio");
                println!("Parágrafo obtido: {}", paragrafo);
            },
            Err(e) => {
                panic!("Erro ao obter parágrafo: {}", e);
            }
        }
    }

    #[test]
    fn test_adicionar_pais() {
        // Preparação do ambiente de teste
        let mut paises = Vec::new();
        let nome_pais = "Brasil";
        let continente = "América do Sul";

        match adicionar_pais(&mut paises) {
            Ok(()) => {
                assert_eq!(paises.len(), 1, "A lista de países não contém o país adicionado");
                let pais_adicionado = &paises[0];
                assert_eq!(pais_adicionado.nome, nome_pais, "O nome do país adicionado não corresponde ao esperado");
                assert_eq!(pais_adicionado.continente, continente, "O continente do país adicionado não corresponde ao esperado");
            },
            Err(e) => {
                panic!("Erro ao adicionar país: {}", e);
            }
        }
    }
}