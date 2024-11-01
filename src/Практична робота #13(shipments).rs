// Написати функцію яка рахує мінімальну кількість переносу грузу щоб на всіх кораблях був однаковий груз
   // fn count_permutation(shipments: &Vec<u32>) -> usize


fn count_permutation(shipments: &Vec<u32>) -> isize {
    let sum: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if sum % n != 0 {
        return -1; // Невозможно сделать веса равными на всех кораблях
    }

    let target_weight = sum / n;
    let mut moves = 0;
    let mut imbalance = 0;

    for &weight in shipments {
        imbalance += weight as isize - target_weight as isize;
        moves += imbalance.abs();
    }

    moves
}

fn main() {
    let shipments = vec![10, 20, 30, 40, 50];
    let result = count_permutation(&shipments);
    println!("Minimum moves: {}", result);
}


// написати функцію генерації Vec<32> які можуть бути розподілені однаково.
    //fn gen_shipments(n: usize) -> Vec<u32>


use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let total_weight = rng.gen_range(n * 10..n * 100);
    let avg_weight = total_weight / n as u32;
    let mut shipments = vec![avg_weight; n];
    let mut extra = total_weight % n as u32;

    // Розподіляємо залишок рівномірно між першими елементами
    for i in 0..extra as usize {
        shipments[i] += 1;
    }

    shipments
}

fn count_permutation(shipments: &Vec<u32>) -> isize {
    let sum: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if sum % n != 0 {
        return -1; // Неможливо зробити ваги однаковими на всіх кораблях
    }

    let target_weight = sum / n;
    let mut moves = 0;
    let mut imbalance = 0;

    for &weight in shipments {
        imbalance += weight as isize - target_weight as isize;
        moves += imbalance.abs();
    }

    moves / 2
}

fn main() {
    let shipments = gen_shipments(10);
    println!("Generated shipments: {:?}", shipments);

    let moves = count_permutation(&shipments);
    println!("Minimum moves: {}", moves);
}
