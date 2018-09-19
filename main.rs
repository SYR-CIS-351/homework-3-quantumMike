//Michael Hellman's homework 1
use std::cmp;

fn main() {
    let initial = [66, 70, 52, 93, 44, 67, 47, 10, 11, 13, 94, 99, 51, 12];
    let mut to_sort;
    println!("initial:          {:?}",initial);

    to_sort  = initial.clone();
    bubble_sort(&mut to_sort);
    println!("bubble-sorted:    {:?}",to_sort);

    to_sort  = initial.clone();
    sel_sort(&mut to_sort);
    println!("selection-sorted: {:?}",to_sort);

    to_sort  = initial.clone();
    insert_sort(&mut to_sort);
    println!("insertion-sorted: {:?}",to_sort);

    println!();
    println!("unordered search:");
    report_search(44,unordered_search(44,&initial[..]));
    report_search(43,unordered_search(43,&initial[..]));

    println!();
    println!("binary search:");
    report_search(44,binary_search(44,&to_sort[..]));
    report_search(43,binary_search(43,&to_sort[..]));

    println!();
    println!("the min and max of initial are {:?}",
             min_max(&initial[..]));
}

/*
// NOTE!! The following will not compile: It needs lifetime annotations.
// We'll fix this later on.
fn swap(x :&mut u32, y : &mut u32) {
    let t = x;
    x = y;
    y = t;
}
*/

// MOVED THE HOMEWORK FUNCTIONS UP THE PAGE

// Selection Sort Implementation
fn sel_sort(a : &mut [u32]) {
    let len = a.len();
    for i in 0..len {
        let mut min = i;
        for j in (i+1)..len {
            if a[j] < a[min]{
                min = j;
            }
            if min != i {
                let t = a[i];
                a[i] = a[min];
                a[min] = t;
            }
        }
    }

}


// Insertion Sort Implementation
fn insert_sort(a : &mut [u32]) {
    let len = a.len();
    let mut i = 0;

    while i < len {
        let x = a[i];
        let mut j = i;

        while j > 0 && a[j-1] > x {
            let g = a[j];
            a[j] = a[j-1];
            a[j-1] = g;
            j -= 1;
        }
        i += 1;
    }
}


// Binary Search Implementation
fn binary_search(x : u32, a : &[u32]) -> Option<usize> {
    let len = a.len();
    let mut l = 0 ;
    let mut r = len - 1;

    while l <= r {
        let m = (l+r)/2;
        if a[m] < x {
            l = m + 1;
        }
        else if a[m] > x {
            r = m - 1;
        }
        else {
            return Some(m);
        }
    }

    None
}

fn min_max(a : &[u32]) -> (u32,u32) {
    let len = a.len();
    let mut pair1  : (u32, u32);
    let mut pair2  : (u32, u32);
    assert!(len>0);


    if len < 2 {
        return (cmp::max(a[0], a[len-1]), cmp::min(a[0], a[len-1]));
    }
    else {
        pair1 = min_max(&a[0..len/2]);
        pair2 = min_max(&a[len/2..len]);
    }

    return (cmp::min(pair1.0, pair2.0), cmp::max(pair1.1, pair2.1));
}

// NOTE:
// cmp::min(a,b) returns the minimum of a and b
// cmp::max(a,b) returns the maximum of a and b




fn bubble_sort(a : &mut [u32]) {
    let len = a.len();
    for i in 0..len {
        for j in 0..(len-i-1) {
            if a[j]>a[j+1] {
                // swap the values of a[j] and a[j+1]
                let t = a[j];
                a[j] = a[j+1];
                a[j+1] = t;
            }
        }
    }
}

fn report_search(x : u32, r : Option<usize>) {
    print!("\t {} ",x);
    match r {
        None    => { println!("not found"); },
        Some(i) => { println!("found at index {}",i); },
    }
}

fn unordered_search(x : u32, a : &[u32]) -> Option<usize> {
    for i in 0..a.len() {
        if x==a[i] { return Some(i); }
    }
    None
}
