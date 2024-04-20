use std::fs::File;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use urlencoding::encode;
use scraper::{Html, Selector};
use reqwest::blocking::get;

pub fn ver_fato_aleatorio_de_pais() -> Result<String, Box<dyn std::error::Error>> {
    // Abra o arquivo de dados CSV e leia os países
    let file = File::open("data/dados.csv")?;
    let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_reader(file);

    // Coleta os nomes dos países
    let mut paises = Vec::new();
    for result in reader.records() {
        let record = result?;
        if let Some(nome_pais) = record.get(0) {
            paises.push(nome_pais.to_string());
        }
    }

    // Se não houver países no arquivo, retorna um erro
    if paises.is_empty() {
        return Err("Lista de países vazia".into());
    }

    // Seleciona aleatoriamente um país da lista
    let pais_aleatorio = paises.choose(&mut rand::thread_rng()).unwrap();

    // Formata o nome do país para que possa ser usado como parte da URL do Wikipedia
    let binding = pais_aleatorio.replace(" ", "_");
    let nome_pais_formatado = encode(&binding);

    // Construa a URL completa usando o nome formatado do país
    let url = format!("https://pt.wikipedia.org/wiki/{}", nome_pais_formatado);

    // Chamando a função obter_paragrafo_wikipedia e retornando o resultado
    obter_paragrafo_wikipedia(&url)
}

fn obter_paragrafo_wikipedia(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Faça a requisição GET para a URL da Wikipedia
    let response = get(url)?;
    let body = response.text()?;

    // Parseie o HTML usando a biblioteca scraper
    let document = Html::parse_document(&body);

    // Use um seletor CSS para encontrar o primeiro parágrafo na página
    let selector = Selector::parse("p").unwrap();
    if let Some(paragrafo) = document.select(&selector).next() {
        // Extraia o texto do parágrafo
        let texto_paragrafo = paragrafo.text().collect::<Vec<_>>().join(" ");
        Ok(texto_paragrafo)
    } else {
        Err("Nenhum parágrafo encontrado".into())
    }
}