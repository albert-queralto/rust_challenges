fn generate_spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut num = 1;
    let (mut top, mut bottom, mut left, mut right) = (0, size - 1, 0, size - 1);

    while top <= bottom && left <= right {
        // Traverse from left to right
        for i in left..=right {
            matrix[top][i] = num;
            num += 1;
        }
        top += 1;

        // Traverse from top to bottom
        for i in top..=bottom {
            matrix[i][right] = num;
            num += 1;
        }
        right -= 1;

        // Traverse from right to left
        if top <= bottom {
            for i in (left..=right).rev() {
                matrix[bottom][i] = num;
                num += 1;
            }
            bottom -= 1;
        }

        // Traverse from bottom to top
        if left <= right {
            for i in (top..=bottom).rev() {
                matrix[i][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

    matrix
}

fn main() {
    let size = 5;
    let matrix = generate_spiral_matrix(size);
    for row in matrix {
        for val in row {
            print!("{:3} ", val);
        }
        println!();
    }
}