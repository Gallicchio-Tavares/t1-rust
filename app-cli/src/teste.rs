// mod paises;

// use serde::Deserialize;
// use std::collections::HashMap;

// #[derive(Debug, Deserialize)]
// struct Continente {
//     #[serde(rename = "continente")]
//     continente: String,
//     paises: Vec<String>,
// }

// fn main() {
//     let the_file = r#"
//         [
//             {
//                 "continente": "Ásia",
//                 "paises": [
//                     "Afeganistão", "Arábia Saudita", "Bangladesh", "Barein", "Brunei", "Butão", "Camboja", "Catar", "Cazaquistão", "China", "Cingapura", "Coreia do Norte", "Coreia do Sul", "Emirados Árabes Unidos", "Filipinas", "Iêmen", "Índia", "Indonésia", "Irã", "Iraque", "Israel", "Japão", "Jordânia", "Kuwait", "Laos", "Líbano", "Malásia", "Maldivas", "Myanmar", "Mongólia", "Nepal", "Omã", "Paquistão", "Quirguistão", "Síria", "Sri Lanka", "Tadjiquistão", "Tailândia", "Timor-Leste", "Turquia", "Turcomenistão", "Uzbequistão", "Vietnã"
//                 ]
//             },
//             {
//                 "continente": "África",
//                 "paises": [
//                     "África do Sul", "Angola", "Argélia", "Benin", "Botsuana", "Burkina Fasso", "Burundi", "Cabo Verde", "Camarões", "Chade", "Comores", "Congo", "Costa do Marfim", "Djibuti", "Egito", "Eritreia", "Etiópia", "Gabão", "Gâmbia", "Gana", "Guiné", "Guiné-Bissau", "Guiné-Equatorial", "Lesoto", "Libéria", "Líbia", "Madagáscar", "Malauí", "Mali", "Marrocos", "Maurício", "Mauritânia", "Moçambique", "Namíbia", "Níger", "Nigéria", "Quênia", "República Centro-Africana", "República Democrática do Congo", "Ruanda", "São Tomé e Príncipe", "Seicheles", "Senegal", "Serra Leoa", "Somália", "Sudão", "Sudão do Sul", "Suazilândia", "Tanzânia", "Togo", "Tunísia", "Uganda", "Zâmbia", "Zimbábue"
//                 ]
//             },
//             {
//                 "continente": "Europa",
//                 "paises":  [
//                     "Albânia", "Alemanha", "Andorra", "Armênia", "Áustria", "Azerbaijão", "Bélgica", "Bósnia-Herzegóvina", "Bulgária", "Croácia", "Dinamarca", "Eslováquia", "Eslovênia", "Espanha", "Estônia", "Finlândia", "França", "Geórgia", "Grécia", "Hungria", "Irlanda", "Islândia", "Itália", "Letônia", "Liechtenstein", "Lituânia", "Luxemburgo", "Macedônia", "Malta", "Moldávia", "Mônaco", "Montenegro", "Noruega", "Países Baixos", "Polônia", "Portugal", "Reino Unido", "República Checa", "Romênia", "Rússia", "San Marino", "Sérvia", "Suécia", "Suíça", "Ucrânia"
//                 ]
//             },
//             {
//                 "continente": "Oceania",
//                 "paises": [
//                     "Austrália", "Fiji", "Ilhas Marshall", "Ilhas Salomão", "Kiribati", "Micronésia", "Nauru", "Nova Zelândia", "Palau", "Papua Nova Guiné", "Samoa", "Tonga", "Tuvalu", "Vanuatu"
//                 ]
//             }
//         ]
//     "#;

//     let continentes: Vec<Continente> = serde_json::from_str(the_file).expect("JSON was not well-formatted");
//     println!("{:?}", continentes);
// }
