fn make_same_lenght(left:&mut Vec<i128>,right:&mut Vec<i128>)  {

    if left.len() == right.len() {
        return;
    }

    if left.len() > right.len() {

        let zeroes = left.len() - right.len();

        for _ in 0..zeroes {
            right.push(0);
        }

    }
    else {
        let zeroes = right.len() - left.len();

        for _ in 0..zeroes {
            left.push(0);
        }
    }

}

fn calculate_distances(left:Vec<i128>,right:Vec<i128>) -> i128 {

    let mut distances: Vec<i128> = vec![0; left.len()];

    for i in 0..left.len() {

        if left[i] == right[i] {
            continue;
        }
        else if left[i] > right[i] {
            distances[i] = left[i] - right[i];
        }
        else {
            distances[i] = right[i] - left[i];
        }

    }
    let sum: i128 = distances.iter().sum();

    sum
}

fn main() {
    let mut left_list: Vec<i128> = vec![3,4,2,1,3,3];

    let mut right_list: Vec<i128> = vec![4, 3, 5, 3, 9, 3];

    left_list.sort();
    right_list.sort();

    make_same_lenght(&mut left_list,&mut right_list);

    println!("{:?}",left_list);
    println!("{:?}",right_list);

   let result:i128 = calculate_distances(left_list,right_list);

   println!("{:?}",result);
}


