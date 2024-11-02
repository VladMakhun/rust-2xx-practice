use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(rectangles: &[Rectangle]) -> i32 {
    // Знаходимо межі області, яку потрібно розглядати
    let min_x = rectangles.iter().map(|r| r.a.x).min().unwrap();
    let max_x = rectangles.iter().map(|r| r.b.x).max().unwrap();
    let min_y = rectangles.iter().map(|r| r.b.y).min().unwrap();
    let max_y = rectangles.iter().map(|r| r.a.y).max().unwrap();

    // Створюємо растр і множину для відстеження відвіданих точок
    let mut raster: HashSet<Point> = HashSet::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for rect in rectangles {
                if rect.a.x <= x && x <= rect.b.x && rect.b.y <= y && y <= rect.a.y {
                    raster.insert(Point { x, y });
                    break;
                }
            }
        }
    }

    raster.len() as i32
}

fn main() {
    let data = vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ];

    let occupied = area_occupied(&data);
    println!("Occupied area: {}", occupied);
}
