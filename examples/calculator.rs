use algods::data_structure::ListStack;

// Simulating a relatively simple calcutor
// implemented with the Djikstra two-stack algorithm
#[derive(Debug)]
pub struct Calculator<T> {
    ops: ListStack<String>,
    vals: ListStack<T>,
}
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::str::FromStr,
    > Default for Calculator<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<
        T: std::ops::Add<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::str::FromStr,
    > Calculator<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            ops: ListStack::new(),
            vals: ListStack::new(),
        }
    }

    pub fn compute(&mut self, expression: String) -> T {
        // operations, parentheses and operands should be separated
        // by white spaces, e.g.  ( ( 1 * ( 2 + 3 ) ) + ( 4 * ( 5 + 6 ) ) )
        for elt in expression.split_whitespace() {
            let c = elt.to_string();

            if c == "+" || c == "*" || c == ":" || c == "/" {
                self.ops.push(c);
            } else if c == ")" {
                // Apply the last operation to the 2 last values
                let op = self.ops.pop().expect("Failed poping last op");
                let a = self.vals.pop().unwrap();
                let b = self.vals.pop().unwrap();
                if op == "+" {
                    // println!("{}", a + b);
                    self.vals.push(a + b);
                } else if op == "*" {
                    // println!("{}", a * b);
                    self.vals.push(a * b);
                } else if op == ":" || op == "/" {
                    self.vals.push(b / a);
                }
            } else if c == "(" {
            } else {
                self.vals.push(c.parse::<T>().unwrap());
            }
        }

        let res = self.vals.pop().expect("Failed poping result");
        res
    }
}

fn main() {
    let mut e = Calculator::<u8>::new();
    let exp = "( ( 1 * ( 2 + 3 ) ) + ( ( 4 * ( 5 + 6 ) ) + ( 10 : ( 4 + 1 ) ) ) )".to_string();
    let res = e.compute(exp);
    assert_eq!(res, 51);
}
