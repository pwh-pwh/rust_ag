pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut result = vec![Vec::<u32>::new(); row_count as usize];
        for i in 0..row_count {
            let i = i as usize;
            result[i] = vec![0; i + 1];
            result[i][0] = 1;
            result[i][i] = 1;
            for j in 1..i {
                result[i][j] = result[i - 1][j - 1] + result[i - 1][j];
            }
        }
        Self(result)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
