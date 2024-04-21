use std::fs::File;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use urlencoding::encode;
use scraper::{Html, Selector};
use reqwest::blocking::get;

pub fn ver_fato_aleatorio_de_pais() -> Result<(String, String), Box<dyn std::error::Error>> {

    let file = File::open("data/dados.csv")?;
    let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_reader(file);

    let mut paises = Vec::new();
    for result in reader.records() {
        let record = result?;
        if let Some(nome_pais) = record.get(0) {
            paises.push(nome_pais.to_string());
        }
    }

    if paises.is_empty() {
        return Err("Lista de países vazia".into());
    }

    let pais_aleatorio = paises.choose(&mut rand::thread_rng()).unwrap().clone();
    let binding = pais_aleatorio.replace(" ", "_");
    let nome_pais_formatado = encode(&binding);

    let url = format!("https://pt.wikipedia.org/wiki/{}", nome_pais_formatado);

    Ok((pais_aleatorio, url))
}

pub fn obter_paragrafo(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Fazer uma requisição GET para obter o HTML da página da Wikipedia
    let response = get(url)?;
    let html = response.text()?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("body p").unwrap();

    let mut paragraphs = Vec::new();
    for paragraph in document.select(&selector) {
        let text = paragraph.text().collect::<String>();
        if !text.is_empty() && text.contains('.') { // Verifica se o parágrafo não está vazio e contém mais de uma frase
            paragraphs.push(text);
        }
    }

    if paragraphs.is_empty() {
        return Err("Nenhum parágrafo válido encontrado".into());
    }

    // Selecionar aleatoriamente um parágrafo
    let mut rng = rand::thread_rng();
    let selected_paragraph = paragraphs.choose(&mut rng).unwrap(); // Usamos unwrap aqui pois sabemos que o vetor `paragraphs` não está vazio
    let clean_paragraph = limpeza(selected_paragraph);

    Ok(clean_paragraph.clone())
}


fn limpeza(paragrafo: &str) -> String {
    let mut limpo = String::new();
    let mut dentro_colchetes = false;

    for c in paragrafo.chars() {
        match c {
            '[' => dentro_colchetes = true,
            ']' => dentro_colchetes = false,
            _ if !dentro_colchetes => limpo.push(c),
            _ => (),
        }
    }

    limpo
}