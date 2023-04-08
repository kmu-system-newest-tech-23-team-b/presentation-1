/// 실습 1
fn selection_sort(arr: &mut [i32]) {
    let arr_length = arr.len();
    for i in 0..arr_length {
        let mut min_index = i;
        for j in (i + 1)..arr_length {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn main() {
    let mut original_arr = [1, 3, 2, 5, 4];
    let mut copy = original_arr.clone();
    selection_sort(&mut copy);
    original_arr.sort();
    print!("{}", original_arr == copy);
}