// Iterators is lazy have no effect until you call methods and consume the iterator to use it up.

fn main() {
    let nums = vec![1,2,3];

    for value in nums{
        println!("{}", value)
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("Got: {val}")
    }
}

fn main() {
    let mut v1 = vec![1, 2, 3];

    let v1_iter = v1.iter_mut();

    for val in v1_iter {
        *val = *val + 1;
    }
    println!("{:?}", v1)
}

fn main() {
    let  v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    while let Some(val) = v1_iter.next() {
        println!("{}", val);
    }

    println!("{:?}", v1)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.into_iter(); 

    for val in v1_iter {
        println!("{}", val);
    }

    println!("{}", v1);
} // gotta sacrifice the last v1 as the ownership is with the iterator

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); 

    let total: i32 = v1_iter.sum();
    println!("{}", total);
} // consuming adapter - iterator the lower iterator is using the sum one

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); 

    let iter2 = v1_iter.map(|x| x + 1);
    for x in iter2 {
        println!("{}", x);
    }
    println!("{:?}", v1);
}
// Iterator adapter they dont consume the iterator instead 
// they produce different iterators by changing some aspect of original.
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); 

    let iter2 = v1_iter.filter(|x| *x % 2 ==0);
    for x in iter2 {
        println!("{}", x);
    }
    println!("{:?}", v1);
} //filter