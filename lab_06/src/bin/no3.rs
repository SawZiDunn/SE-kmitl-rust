use std::cmp::Ordering;

trait SimpleAnalyzable {
    fn mean(&self) -> f64;
    fn median(&self) -> f64;
}

impl SimpleAnalyzable for Vec<f64> {
    fn mean(&self) -> f64 {
        let sum: f64 = self.iter().sum();
        sum / self.len() as f64
    }

    fn median(&self) -> f64 {
        let mut sorted = self.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let len = sorted.len();
        if len % 2 == 0 {
            (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
        } else {
            sorted[len / 2]
        }
    }
}

#[derive(Debug)]
struct SimpleDataSet {
    data: Vec<f64>,
}

impl SimpleDataSet {
    fn new(data: Vec<f64>) -> Self {
        SimpleDataSet { data: data } // or just { data }
    }

    fn filter<F>(&self, predicate: F) -> Self
    where
        F: Fn(&f64) -> bool,
    {
        SimpleDataSet {
            data: self.data.iter().cloned().filter(predicate).collect(),
        }
    }
}

impl SimpleAnalyzable for SimpleDataSet {
    fn mean(&self) -> f64 {
        self.data.mean()
    }

    fn median(&self) -> f64 {
        self.data.median()
    }
}

fn main() {
    let data = SimpleDataSet::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    println!("Mean: {}", data.mean());
    println!("Median: {}", data.median());

    let filtered_data = data.filter(|&x| x > 4.0);
    print!("{:#?}", filtered_data);
    println!("Filtered Mean: {}", filtered_data.mean());
    println!("Filtered Median: {}", filtered_data.median());
}
