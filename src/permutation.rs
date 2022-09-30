pub struct Permutations<T: Clone> {
    a: Vec<T>,
    n: usize,
    p: Vec<usize>,
    i: usize,
    initial_call: bool,
}

impl<T: Clone> Permutations<T> {
    pub fn new(a: &[T]) -> Self {
        let a = a.to_vec();
        let n = a.len();
        let p = create_integer_array(n);
        let i = 1;
        let initial_call = true;
        Permutations {
            a,
            n,
            p,
            i,
            initial_call,
        }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    // quickperm see https://www.baeldung.com/cs/array-generate-all-permutations
    // and https://www.quickperm.org/
    fn next(&mut self) -> Option<Self::Item> {
        if self.initial_call {
            self.initial_call = false;
            Some(self.a.to_vec())
        } else if self.i < self.n {
            self.p[self.i] -= 1;
            let j = if odd(self.i) { self.p[self.i] } else { 0 };
            self.a.swap(j, self.i);
            self.i = 1;
            while self.p[self.i] == 0 {
                self.p[self.i] = self.i;
                self.i += 1;
            }
            Some(self.a.to_vec())
        } else {
            None
        }
    }
}

fn create_integer_array(length: usize) -> Vec<usize> {
    (0..length + 1).collect::<Vec<usize>>()
}

fn odd(n: usize) -> bool {
    n % 2 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_integer_array() {
        let v = create_integer_array(3);
        assert_eq!(vec![0, 1, 2, 3], v)
    }

    #[test]
    fn test_odd() {
        assert_eq!(odd(1), true);
        assert_eq!(odd(2), false);
    }

    #[test]
    fn test_permutations() {
        let mut p = Permutations::new(&vec![1, 2, 3]);
        assert_eq!(p.next(), Some(vec![1, 2, 3]));
        assert_eq!(p.next(), Some(vec![2, 1, 3]));
        assert_eq!(p.next(), Some(vec![3, 1, 2]));
        assert_eq!(p.next(), Some(vec![1, 3, 2]));
        assert_eq!(p.next(), Some(vec![2, 3, 1]));
        assert_eq!(p.next(), Some(vec![3, 2, 1]));
        assert_eq!(p.next(), None);
    }
}
