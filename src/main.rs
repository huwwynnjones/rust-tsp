mod permutation;

use crate::permutation::Permutations;

fn main() {
    //let result = permutations(&mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    //let result = permutations(&mut vec![1, 2, 3]);
    //println!("{:?}", result);
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let perm = Permutations::new(&a);
    println!("{}", perm.count());
}

// quickperm see https://www.baeldung.com/cs/array-generate-all-permutations
// and https://www.quickperm.org/
fn permutations<T: Clone>(a: &mut [T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let n = a.len();
    let mut p = create_integer_array(n);
    result.push(a.to_vec());
    let mut i = 1;
    while i < n {
        p[i] -= 1;
        let j = if odd(i) { p[i] } else { 0 };
        a.swap(j, i);
        result.push(a.to_vec());
        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }
    result
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
    fn test_permutations_char() {
        let mut a = vec!['a', 'b', 'c'];
        permutations(&mut a);
        println!("{:?}", a);
        //assert_eq!(permutations(&mut a), vec![vec![]])
    }

    #[test]
    fn test_permutations_int() {
        let mut a = vec![1, 2, 3];
        assert_eq!(
            permutations(&mut a),
            vec![
                vec![1, 2, 3],
                vec![2, 1, 3],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 3, 1],
                vec![3, 2, 1]
            ]
        )
    }
}
