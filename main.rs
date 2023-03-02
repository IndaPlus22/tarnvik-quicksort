/**
 * Cute sort, quick sort 
 * Author: Tilde Tärnvik <tarnvik@kth.se> 
 */

 use std::io::{self};
 use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    let mut values = input
        .lock()
        .lines()
        .next().unwrap().unwrap()
        .split_whitespace()
        .skip(1)
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    
    quicksort(&mut values);

    for value in values{
        print!("{} ", value);
    }
}

fn insertion_sort(values: &mut [i32]) {
    for n in 1..values.len() {
        let mut i = n;
        while i > 0 && values[i - 1] > values[i] {
            values.swap(i, i-1);
            i -= 1;
        }
    }
}

/**
 * Sorts value of i32 slice using recursive quicksort 
 */
fn quicksort(values: &mut [i32]) {
    let length = values.len();

    if length <= 1 {
        return;
    } else if length < 103 {
        insertion_sort(values);
    } else {
        let partition_index = hoare_partition(values);

        quicksort(&mut values[..partition_index+1]);
        quicksort(&mut values[partition_index+1..]);
    }
}

/**
 * [basic hoare partition]
 */
fn hoare_partition (values: &mut [i32]) -> usize {
    let length = values.len(); 

    let mut i: isize = -1;
    let mut j = length;

    let pivot_index = tukeys_ninther(values);
    values.swap(0, pivot_index);
    let pivot = values[0];

    loop {

        i += 1;
        while values[i as usize] < pivot{
            i += 1;
        }

        j -= 1;
        while values[j] > pivot {
            j -= 1;
        }

        if i as usize >= j{
            return j;
        }

        values.swap(i as usize, j);
    }
}

/**
 * [Returns the median of three values in an i32 slice]
 */
fn median_of_3 (values: &[i32], a: usize, b: usize, c: usize) -> usize{
    /*
     b a c
     b c a
     c b a 
     */
    if values[a] < values[b] {
        if values[b] < values[c]{
            b
        } else if values[c] < values[a]{
            a
        } else {
            c
        }
    /*
     a b c 
     a c b
     c a b
     */
    } else {
        if values[a] < values[c]{
            a
        } else if values[c] < values[b]{
            b
        } else {
            c
        }
    }
}

/**
 * Uses tukeys_ninther approach to return the median of nine values within an i32 slice
 */
fn tukeys_ninther(values: &[i32]) -> usize{
    let length = values.len(); 

    let lo = 0;
    let hi = length - 1;
    let mid = hi/2;
    let delta = hi/8;

    let m1 = median_of_3(values, lo, lo+delta, lo+2*delta);
    let m2 = median_of_3(values, mid-delta, mid, mid+delta);
    let m3 = median_of_3(values, hi-2*delta, hi-delta, hi);
    
    median_of_3(values, m1, m2, m3)
}