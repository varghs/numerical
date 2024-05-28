use std::fmt;

struct Polynomial {
    coefficients: Vec<f64>,
    degrees: Vec<i32>,
}

#[derive(Debug, Clone)]
struct MismatchError;

impl fmt::Display for MismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coefficient and degree vectors are not of equal length")
    }
}

impl Polynomial {
    fn new(coefficients: Vec<f64>, degrees: Vec<i32>) -> Result<Self, MismatchError> {
        if coefficients.len() != degrees.len() {
            return Err(MismatchError);
        }

        let mut combined: Vec<(i32, f64)> = degrees
            .iter()
            .cloned()
            .zip(coefficients.iter().cloned())
            .collect();

        combined.sort_by(|a, b| a.0.cmp(&b.0));

        let (sorted_degrees, sorted_coefficients): (Vec<i32>, Vec<f64>) =
            combined.into_iter().unzip();

        Ok(Self {
            coefficients: sorted_coefficients,
            degrees: sorted_degrees,
        })
    }

    fn differentiate(&self) -> Polynomial {
        let filtered: Vec<(i32, f64)> = self
            .degrees
            .clone()
            .into_iter()
            .zip(self.coefficients.clone().into_iter())
            .filter(|&(degree, _)| degree != 0)
            .map(|(degree, coefficient)| (degree - 1, coefficient * degree as f64))
            .collect();

        let (degrees, coefficients): (Vec<i32>, Vec<f64>) = filtered.into_iter().unzip();

        Self {
            degrees,
            coefficients,
        }
    }

    fn compute(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .zip(self.degrees.iter())
            .map(|(&coefficient, &degree)| coefficient * x.powi(degree))
            .sum()
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terms: Vec<String> = self
            .coefficients
            .iter()
            .zip(self.degrees.iter())
            .map(|(&coef, &deg)| format!("({}, {})", deg, coef))
            .collect();
        write!(f, "{}", terms.join(", "))
    }
}

fn main() {
    let c: Vec<f64> = (1..10).map(|x| x.into()).collect();
    let d: Vec<i32> = (0..9).collect();
    let Ok(p) = Polynomial::new(c, d) else {
        return;
    };

    let diff_p = p.differentiate();
    println!("{}", p);
    println!("{}", diff_p);

    let c = vec![1.0, 2.0, 1.0];
    let d = vec![2, 1, 0];
    let Ok(p) = Polynomial::new(c, d) else {
        return;
    };
    println!("{}", p.compute(-1.0));

    let diff_p = p.differentiate();
    println!("{}", diff_p.compute(-2.0));

    let double_diff_p = diff_p.differentiate();
    println!("{}", double_diff_p.compute(-2.0));

    let triple_diff_p = double_diff_p.differentiate();
    println!("{}", triple_diff_p.compute(-1.0));
}
