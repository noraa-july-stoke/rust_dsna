pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let rows = board.len();
    let cols = board[0].len();

    fn dfs(
        row: usize,
        col: usize,
        board: &mut Vec<Vec<char>>,
        word: &String,
        s_idx: usize,
    ) -> bool {
        if s_idx == word.len() {
            return true;
        }

        if row < 0 || row >= board.len() || col < 0 || col >= board.len() || board[row][col] != word.chars().nth(s_idx).unwrap() {
            return false;
        }

        let temp: char = board[row][col];
        board[row][col] = '#';

        let res: bool = dfs(row - 1, col, board, word, s_idx + 1)
            || dfs(row, col + 1, board, word, s_idx + 1)
            || dfs(row + 1, col, board, word, s_idx + 1)
            || dfs(row, col - 1, board, word, s_idx + 1);

        board[row][col] = temp;
        return res;
    }

    for row in 0..rows {
        for col in 0..cols {
            let mut s_idx = 0;
            if board[row][col] == word.chars().nth(s_idx).unwrap()
                && dfs(row, col, &mut board.clone(), &word, s_idx)
            {
                return true;
            }
        }
    }
    return false;
}
