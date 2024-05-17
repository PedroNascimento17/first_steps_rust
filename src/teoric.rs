use std::io;

fn _convert_to_int(data_input: &String) -> i32 {
    let x: i32 = data_input.trim().parse::<i32>().unwrap_or_default();
    x
}

fn _greather_or_lower_equal() {
    println!("Informe o valor do primeiro número:");
    let mut number1: String = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");

    println!("Informe o valor do segundo número:");
    let mut number2: String = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2");
    
    let converted_number1: i32 = _convert_to_int(&number1);
    let converted_number2: i32 = _convert_to_int(&number2);

    if converted_number1 == 0 || converted_number2 == 0 {
        println!("Os valores informados inválidos foram substituídos por 0");
    }

    if converted_number1 > converted_number2 {
        println!("O numero {} é maior que {}", converted_number1, converted_number2);
    }
    else {
        println!("O numero {} é menor ou igual que {}", converted_number1, converted_number2);
    }
}

fn _sum_digits_from_number() {
    let mut sum: i32 = 0;
    let mut input_value: String = String::new();

    println!("Informe um número:");
    io::stdin().read_line(&mut input_value).expect("Erro ao ler valor de entrada");

    let mut integer_value: i32 = _convert_to_int(&input_value);

    while integer_value != 0 {
        let mod_by_ten = integer_value % 10;
        sum += mod_by_ten;
        integer_value /= 10;
    } 

    println!("O resultado da soma dos digitos do número {} é {}", input_value, sum);   
}

fn _factorial_of_number() {
    let mut factorial_input: String = String::new(); 
    println!("Informe um número para calcular o fatorial:");  
    io::stdin().read_line(&mut factorial_input).expect("Erro ao ler valor de entrada");

    let mut factorial_input_integer: i32 = _convert_to_int(&factorial_input);
    let mut factorial: i32 = 1;

    while factorial_input_integer > 1 {
        factorial *= factorial_input_integer;
        factorial_input_integer -= 1;
    }

    println!("Fatorial de {} é {}", factorial_input, factorial); 
}

fn _greatest_common_divisor() {
    println!("Informe o valor do primeiro número:");
    let mut number1: String = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number1");

    println!("Informe o valor do segundo número:");
    let mut number2: String = String::new();
    io::stdin().read_line(&mut number2).expect("Erro ao ler number2");

    let mut converted_number1: i32 = _convert_to_int(&number1);
    let mut converted_number2: i32 = _convert_to_int(&number2);

    while converted_number2 != 0 {
        let temp: i32 = converted_number2;
        converted_number2 = converted_number1 % converted_number2;
        converted_number1 = temp;
    }

    println!("O maior divisor comum entre {} e {} é {}", number1.trim(), number2.trim(), converted_number1);
}

fn _double(num: i32) -> i32 {
    2 * num
}

fn _max(number1: i32, number2: i32) -> i32 {
    if number1 >= number2 {
        number1
    }
    else {
        number2
    }
}
