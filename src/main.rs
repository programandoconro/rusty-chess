fn chatgptmain() {
    let mut board: Vec<Vec<i32>> = Vec::new(); // or let mut board = vec![];

    for i in 1..=64 {
        // remainder will be one every 8 times, 1 % 8 == 1, 9 % 8 == 1, 17 % 8 == 1 ...
        if i % 8 == 1 {
            board.push(Vec::new() /*or vec![] */);
        }
        let len: usize = board.len(); // usize is used to hold big or unknow numbers
        board[len - 1].push(i);
    }

    println!("{:?}", board);
}

fn matrix2d() {
    let mut matrix: Vec<Vec<i32>> = Vec::new(); // or vec![]

    for i in 1..=64 {
        if i % 8 == 1 {
            matrix.push(Vec::new());
        }
        let len: usize = matrix.len();
        matrix[len - 1].push(i);
    }
    println!("{:?}", matrix);
}

// this was my first try
fn mymain() {
    let mut board: Vec<Vec<i32>> = Vec::new();
    let mut counter = 0;

    for i in 1..=8 {
        let mut row: Vec<i32> = Vec::new();
        row.push(counter + i);
        for _j in 1..=7 {
            counter = counter + 1;
            row.push(counter + i);
        }
        board.push(row);
    }

    println!("{:?}", board);
}

fn main() {
    mymain();
    chatgptmain();
    matrix2d();
}
