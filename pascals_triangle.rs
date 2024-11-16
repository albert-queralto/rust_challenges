fn generate_pascals_triangle(rows: usize) -> Vec<Vec<u32>> {
    let mut triangle: Vec<Vec<u32>> = Vec::new();

    for i in 0..rows {
        let mut row = vec![1; i + 1];
        for j in 1..i {
            row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
        }
        triangle.push(row);
    }

    triangle
}

fn print_pascals_triangle(triangle: Vec<Vec<u32>>) {
    for row in triangle {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }
}

fn main() {
    let rows = 6;
    let triangle = generate_pascals_triangle(rows);
    print_pascals_triangle(triangle);
}