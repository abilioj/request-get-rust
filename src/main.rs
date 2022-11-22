use std::io::Read;
use log::error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // let cep = "01001000";
    let cep = match std::env::args().nth(1){
        Some(valor) => valor,
        Nome => {
            error!("Error: parâmetro cep não informado!");
            panic!();
        }
    };

    let url = format!("https://viacep.com.br/ws/{}/json/",cep);
    let mut res = reqwest::blocking::get(url)?;

    // println!("Status {}", res.status());
    // println!("Headers {:#?}", res.headers());

    res.copy_to(&mut std::io::stdout());
    Ok(())
}
