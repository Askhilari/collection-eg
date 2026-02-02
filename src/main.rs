use std::{collections::HashMap, hash::Hash};

fn main() {
    let mut list = vec![2,10,9,3,1,1,2,3,1,1];
    println!("Original sequence is: {:?}", list);
    let sorted_list = sort_var(&mut list);
    println!("Sorted Sequence is: {:?}", sorted_list);
    let sequence_median = get_median(&mut list);
    println!("Median of sequence is: {}", sequence_median);
    get_mode(&mut list);
}

// fn sorting_vec(list: &mut Vec<i32>) -> &Vec<i32> {
//     for _ in 0..list.len(){
//         for i in 0..(list.len() - 1){
//             if list.get(i) > list.get(i+1) {
//                 let temp = list[i+1];
//                 list[i+1] = list[i];
//                 list[i] = temp;
//             }
//         }
//     }
//     list
// }


fn sort_var(list: &mut Vec<i32>) -> &Vec<i32> {
    for _ in 0..list.len() {
        for i in 0..(list.len() - 1){
            if list[i] > list[i+1] {
                let temp = list[i];
                list[i] = list[i+1];
                list[i+1] = temp;
            }
        }
    }
    list
}

fn get_median(list: &mut Vec<i32>) -> i32 {
    let list_len = list.len();
    let index_of_median = list_len / 2;
    let mut median = 0;
    if list_len % 2 == 0 {
        median = (list[index_of_median] + list[index_of_median - 1])/2;
    } else {
        median = list[index_of_median];
    }

    median
}

fn get_mode(list: &mut Vec<i32>) {
    let mut map: HashMap<&mut i32, i32> = HashMap::new();
    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mode = map.values().copied().max();
    for (key, val) in map.iter(){
        if val == &mode.unwrap() {
            println!("Number {key} occurs {val} times. Hence the mode is {key}")
        }
    }
    println!("{map:?}");
    match mode {
        Some(mode) => println!("{mode}"),
        None => println!("No number found"),
    }
}
