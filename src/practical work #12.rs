//Написати функцію яка генерує рандомний вектор довжиною 20 зі значеннями [10..99]  
    fn gen_random_vector(n: usize) -> Vec<i32>

    fn min_adjacent_sum(data: &[i32]) -> ??

use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn main() {
    let random_vec = gen_random_vector(20);
    println!("{:?}", random_vec);

    if let Some(min_sum) = min_adjacent_sum(&random_vec) {
        println!("Minimum adjacent sum: {}", min_sum);
    } else {
        println!("Недостатньо елементів для формування пари.");
    }
}

fn min_adjacent_sum(data: &[i32]) -> Option<i32> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
        }
    }

    Some(min_sum)
}

// Написати функцію яка знаходить минімальну пару у Vec<i32>: fn min_adjacent_sum(data: &[i32]) -> ???

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, (i32, i32))> {
    if data.len() < 2 {
        return None; // Недостатньо елементів для утворення пари
    }

    let mut min_sum = data[0] + data[1];
    let mut min_pair = (data[0], data[1]);

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
        }
    }

    Some((min_sum, min_pair))
}

fn main() {
    let data = vec![34, 7, 23, 32, 5, 62];
    if let Some((sum, (a, b))) = min_adjacent_sum(&data) {
        println!("Minimum adjacent sum is {} with pair ({}, {})", sum, a, b);
    } else {
        println!("Not enough elements to form a pair.");
    }
}
