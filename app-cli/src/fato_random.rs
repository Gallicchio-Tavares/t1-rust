use std::fs::File;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use urlencoding::encode;
use scraper::{Html, Selector};
use reqwest::blocking::get;

pub fn ver_fato_aleatorio_de_pais() -> Result<(String, String), Box<dyn std::error::Error>> {
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
    let pais_aleatorio = paises.choose(&mut rand::thread_rng()).unwrap().clone();

    // Formata o nome do país para que possa ser usado como parte da URL do Wikipedia
    let binding = pais_aleatorio.replace(" ", "_");
    let nome_pais_formatado = encode(&binding);

    // Construa a URL completa usando o nome formatado do país
    let url = format!("https://pt.wikipedia.org/wiki/{}", nome_pais_formatado);

    // Retorna o nome do país sorteado e a URL da Wikipedia
    Ok((pais_aleatorio, url))
}

pub fn obter_paragrafo(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Fazer uma requisição GET para obter o HTML da página da Wikipedia
    let response = get(url)?;
    let html = response.text()?;

    // Parsear o HTML usando a biblioteca scraper
    let document = Html::parse_document(&html);

    // Selecionar todos os parágrafos dentro do corpo da página
    let selector = Selector::parse("body p").unwrap();

    // Coletar todos os parágrafos encontrados
    let mut paragraphs = Vec::new();
    for paragraph in document.select(&selector) {
        paragraphs.push(paragraph.text().collect::<String>());
    }

    // Selecionar aleatoriamente um parágrafo entre o terceiro e o vigésimo
    let mut rng = rand::thread_rng();
    let selected_paragraph = paragraphs.choose(&mut rng).ok_or("Nenhum parágrafo encontrado")?;
    let clean_paragraph = limpeza(&selected_paragraph);
    // Retornar o parágrafo selecionado
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