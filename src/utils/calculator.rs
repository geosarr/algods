#[cfg(test)]
mod unit_test;
use crate::data_structure::ListStack;

// Simulating a relatively simple calcutor
// implemented with the Djikstra two-stack algorithm
#[derive(Debug)]
pub struct Calculator {
    ops: ListStack<String>,
    vals: ListStack<usize>,
}
impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
impl Calculator {
    pub fn new() -> Self {
        Self {
            ops: ListStack::new(),
            vals: ListStack::new(),
        }
    }

    pub fn compute(&mut self, expression: String) -> usize {
        // operations, parentheses and operands should be separated
        // by white spaces, e.g.  ( ( 1 * ( 2 + 3 ) ) + ( 4 * ( 5 + 6 ) ) )
        for elt in expression.split_whitespace() {
            let c = elt.to_string();
            // println!("{:?}", self.vals);
            // println!("{:?}", self.ops);

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
                self.vals.push(c.parse::<usize>().unwrap());
            }
            // println!("\n");
        }

        let res = self.vals.pop().expect("Failed poping result");
        println!("{res}");
        res
    }
}
