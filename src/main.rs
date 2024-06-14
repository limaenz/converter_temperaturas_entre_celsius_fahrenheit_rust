use std::io;

fn main() {
    loop {
        let mut opcao = String::new();
        let mut valor = String::new();

        println!("Digite: 1 - Converter de Celsius (°C) para Fahrenheit (°F)");
        println!("Digite: 2 - Converter de Fahrenheit (°F) para Celsius (°C)");
        println!("Digite: 3 - Sair");

        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha na entrada de dados");

        println!("Quantos graus?");

        io::stdin()
            .read_line(&mut valor)
            .expect("Falha na entrada de dados");

        let valor: f32 = valor
            .trim()
            .parse()
            .expect("Entrada inválida. Não é um valor numerico");

        let opcao: i32 = opcao
            .trim()
            .parse()
            .expect("Entrada inválida. Não é um valor numerico");

        if opcao == 1 {
            let calculo = valor * (9.0 / 5.0) + 32.0;
            println!("É {}", calculo);
        } else if opcao == 2 {
            let calculo = (valor - 32.0) * 5.0 / 9.0;
            println!("É {}", calculo);
        } else if opcao == 3 {
            break;
        } else {
            println!("Favor inserir uma opção valida");
        }
    }
}
