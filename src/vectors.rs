fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(6);

    let ans = even_filter(&vec);
    println!("{:?}", ans);

    println!("{:?}", vec)
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}

fn even_values(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}
    

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(22);
    even_values(&mut vec);
    println!("Even numbers is/are {:?}", vec)
}

fn even_values(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}
    

fn main() {
    let mut vec = vec![1, 2 , 3];// its a macro instead of writing 1 by 1 use vec![]
    even_values(&mut vec);
    println!("Even numbers is/are {:?}", vec)
}
