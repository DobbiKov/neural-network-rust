use rand::{thread_rng, Rng};

pub struct Matrix{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Clone for Matrix{
    fn clone(&self) -> Self {
        Self { rows: self.rows.clone(), cols: self.cols.clone(), data: self.data.clone() }
    }
}

impl Matrix{
    pub fn zeros(rows: usize, cols: usize) -> Matrix{
        Matrix { 
            rows: rows, 
            cols: cols, 
            data: vec![vec![0.0; cols]; rows], 
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix{
        let mut res: Matrix = Matrix::zeros(rows, cols);
        let mut rng = thread_rng();

        for i in 0..rows{
            for j in 0..cols{
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        res
    }

    pub fn multiply(first: &Matrix, second: &Matrix) -> Matrix{
        if first.cols != second.rows{
            panic!(
                "Multiplication is impossible!"
            )
        }
        let mut res: Matrix = Matrix::zeros(first.rows, second.cols);
        
        for i in 0..res.rows{
            for j in 0..res.cols{
                let mut sum: f64 = 0.0;
                for k in 0..first.cols{
                    sum += (first.data[i][k] * second.data[k][j]);
                }
                res.data[i][j] = sum;
            }
        }

        res
    }

    pub fn add(first: &Matrix, second: &Matrix) -> Matrix{
        if first.cols != second.cols || first.rows != second.rows{
            panic!("Addition is impossible!")
        }
        let mut res: Matrix = first.clone();

        for i in 0..res.rows{
            for j in 0..res.cols{
                res.data[i][j] += second.data[i][j];
            }
        }

        res
    }

    pub fn substract(first: &Matrix, second: &Matrix) -> Matrix{
        if first.cols != second.cols || first.rows != second.rows{
            panic!("Addition is impossible!")
        }
        let mut res: Matrix = first.clone();

        for i in 0..res.rows{
            for j in 0..res.cols{
                res.data[i][j] -= second.data[i][j];
            }
        }

        res
    }

    pub fn dot_multiply(first: &Matrix, second: &Matrix) -> Matrix{
        if first.cols != second.cols || first.rows != second.rows {
            panic!("Dot multiplication is impossible!")
        }
        let mut res: Matrix = first.clone();

        for i in 0..res.rows{
            for j in 0..res.cols{
                res.data[i][j] *= second.data[i][j];
            }
        }

        res
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix{
        let mut cols: usize = 0;
        if !data.is_empty(){
            cols = data[0].len();
        }
        Matrix { rows: data.len(), cols: cols, data: data }
    }

    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix{

        Matrix::from( 
            (self.data)
            .clone()
            .into_iter()
            .map(
                |row| 
                row
                .into_iter()
                .map(function)
                .collect()
            )
            .collect() 
        )

    }

    pub fn transpose(&self) -> Matrix{
        let mut res: Matrix = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data[j][i] = self.data[i][j];
            }
        }

        res
    }

    pub fn print(&self){
        for i in &self.data{
            print!("[");
            for j in i{
                print!("{} ", j);
            }
            print!("],");
            println!("");
        }
    }
}
