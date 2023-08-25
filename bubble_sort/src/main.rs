fn main() {
    println!("bubble_sort_i32");
    let mut arr = [5, 4, 3, 2, 1];
    println!("bubble_sort_i32_before: {:?}", arr);
    bubble_sort_i32(&mut arr);
    println!("bubble_sort_i32_after: {:?}", arr);

    let mut arr = [
        Person { id: 5 },
        Person { id: 4 },
        Person { id: 3 },
        Person { id: 2 },
        Person { id: 1 },
    ];
    println!("bubble_sort_partial_ord_before: {:?}", arr);
    bubble_sort_partial_ord(&mut arr);
    println!("bubble_sort_partial_ord_before: {:?}", arr);
}

fn bubble_sort_i32(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Person {
    id: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

fn bubble_sort_partial_ord<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble1() {
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort_i32(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble2() {
        let mut arr = [
            Person { id: 5 },
            Person { id: 4 },
            Person { id: 3 },
            Person { id: 2 },
            Person { id: 1 },
        ];
        bubble_sort_partial_ord(&mut arr);
        assert_eq!(
            arr,
            [
                Person { id: 1 },
                Person { id: 2 },
                Person { id: 3 },
                Person { id: 4 },
                Person { id: 5 },
            ]
        );
    }
}
