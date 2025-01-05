use std::fs;

fn calculate_safety_levels(vecs:&Vec<Vec<i128>>) -> i128 {

    let mut safe_levels = 0;

    for vec in vecs.iter() {

        let mut increasing:bool = false;
        let mut decreasing:bool = false;

        let mut safe:bool = true;

        for i in 0..vec.len() {

            //Pulando a ultima iteração pois abaixo temos diversas expressões que checam o index + 1;
            if i == vec.len() - 1 {continue;}

            //Checando se esta aumentando
            if vec[i] < vec[i+1] {increasing=true;}
            //Checando se esta diminuindo
            else if vec[i] > vec[i+1] {decreasing=true;}

            //Checando se os valores são iguais
            if vec[i] == vec[i +1] {safe = false;}
            //Significa que o vetor estava aumentando e diminuiu.
            else if (increasing) && (vec[i] > vec[i+1])  {safe = false;}
            //Significa que o vetor estava diminuindo e aumentou.
            else if (decreasing) && (vec[i] < vec[i + 1])  {safe = false;}

            //Se estiver aumentando e o valor do indice atual menos o proximo for maior que três
            if (increasing) && ((vec[i] - vec[i + 1]).abs() > 3) {safe = false;}
            //Se estiver diminuindo e o valor do proximo indice menos o indice atual
            else if (decreasing) && ((vec[i +1] - vec[i]).abs() > 3) {safe = false;}


        }

        if safe {
            safe_levels = safe_levels + 1;
        }
    }

    safe_levels
}

fn split_lists(content:&String) ->  Vec<Vec<i128>>  {

    //Separa cada linha do arquivo em um vetor de strings.
    let array_content:Vec<&str> = content.split("\r\n").collect();

    let mut lists: Vec<Vec<i128>> = vec!();

    //Para cada linha do arquivo
    for line in array_content {

        //Encontra um espaço e separa em vetor de referencias para as strings.
        let contents = line.split(" ").collect::<Vec<&str>>();

        //Vetor para guardar cada número dentro da linha.
        let mut list_of_nums : Vec<i128> = vec!();

        //Para cada numero.
        for num in contents {
            //Transforma a referencia a str para uma String e parseia ela em i128
            list_of_nums.push(num.to_string().parse::<i128>().unwrap());
        }

        //Adiciona a lista de numeros coletados ao resultado.
        lists.push(list_of_nums);

    }

    lists
}

fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Deveriamos conseguir abrir o arquivo input.txt");


    let vecs = split_lists(&contents);


    let safe_levels = calculate_safety_levels(&vecs);

    println!("{}", safe_levels);
}
