fn solve_cryptarithm() -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut solutions = Vec::new();
    for m in 1..10 {
        for u in 1..10 {
            if u == m { continue; }
            for x in 1..10 {
                if x == m || x == u { continue; }
                for a in 1..10 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..10 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..10 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..10 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..10 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    if muxa * a == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    solutions
}

fn main() {
    let solutions = solve_cryptarithm();
    for (m, u, x, a, s, l, o, n) in &solutions {
        println!(
            "  {}{}{}{}\n*        {}\n  ------\n    {}{}{}{}",
            m, u, x, a, a, s, l, o, n
        );
    }
    println!("Number of solutions: {}", solutions.len());
}
