/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd + Copy + Clone>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }

    let mid = array[(array.len() - 1) / 2];
    let mut i = -1i32;
    let mut j = array.len() as i32;

    while i < j {
        i += 1;
        while array[i as usize] < mid {
            i += 1;
        }

        j -= 1;
        while array[j as usize] > mid {
            j -= 1;
        }

        if i < j {
            let x = array[i as usize];
            array[i as usize] = array[j as usize];
            array[j as usize] = x;
        }
    }

    sort(&mut array[..=j as usize]);
    sort(&mut array[((j + 1) as usize)..]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
