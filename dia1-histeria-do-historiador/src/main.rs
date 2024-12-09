use std::fs;
use std::iter::zip;

fn calculate_distances(left:Vec<i128>,right:Vec<i128>) -> i128 {

    let mut distance: i128 = 0;

    for (l,r) in zip(left,right) {

        let result = l - r;

        distance = distance + result.abs();

    }

    distance
}


fn split_lists(content:String) -> (Vec<String>,Vec<String>)  {

    //Separa cada linha do arquivo em um vetor de strings.
    let array_content:Vec<&str> = content.split("\r\n").collect();

    //Variaveis para guardar os numeros em strings.

    let mut left_list_string:Vec<String> = vec![];

    let mut right_list_string:Vec<String> = vec![];

    //Para cada linha do arquivo
    for content in &array_content {

        //Encontra um espaço e separa em um array.
        let content = content.split("   ").collect::<Vec<&str>>();

        //Pega o primeiro valor do array a cima e salva na lista correta.
        //Index 0 são os numeros da esquerda.
        left_list_string.push(content[0].to_string());
        //Index 1 são os números da direita.
        right_list_string.push(content[1].to_string());

    }

    (left_list_string,right_list_string)
}

fn split_numbers(left:Vec<String>,right:Vec<String>) -> (Vec<i128>,Vec<i128>) {

    let iter = zip(left, right);

    let mut left_result : Vec<i128> = vec![];

    let mut right_result : Vec<i128> = vec![];

    //Colocando as string dentro dos vetores como números.
    for (l,r) in iter {


        left_result.push(l.parse::<i128>().unwrap());

        right_result.push(r.parse::<i128>().unwrap());

        left_result.sort();
        right_result.sort();

    }

    (left_result,right_result)
}

fn main() {


    let contents = fs::read_to_string("input.txt")
        .expect("Deveriamos conseguir abrir o arquivo input.txt");


    let (left_string_vec,right_string_vec) = split_lists(contents);

    let (left_int_vec,right_int_vec) = split_numbers(left_string_vec,right_string_vec);

    let result:i128 = calculate_distances(left_int_vec,right_int_vec);

    println!("{}", result);
}


