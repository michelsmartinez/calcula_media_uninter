use std::io;

fn main() {
    let mut apols = String::new();
    let mut ap = String::new();
    let mut pd = String::new();
    let mut po = String::new();

    println!("Digite a média das Apols");
    io::stdin()
        .read_line(&mut apols)
        .expect("Falha ao ler média das apols");

    println!("Digite a nota da atividade prática");
    io::stdin()
        .read_line(&mut ap)
        .expect("Falha ao ler nota da atividade prática");

    println!("Digite a nota na prova objetiva");
    io::stdin().read_line(&mut po).expect("Falha ao ler po");

    println!("Digite a a nota na prova discursiva");
    io::stdin().read_line(&mut pd).expect("Falha ao ler pd");

    let apols: u32 = match apols.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let ap: u32 = match ap.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let pd: u32 = match pd.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let po: u32 = match po.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let md: u32 = (po * 3 + apols * 2 + ap * 2 + pd * 3) / 10;
    println!("Sua média até então é: {}", md);
}
