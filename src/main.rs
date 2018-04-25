use std::fmt; // needed for the Display trait
use fmt::Display; // so we don't have to keep typing fmt::Display
//use std::ops::Deref; // we we can ensure that we're only brining in de refrencable stuff in rust
extern crate rand; // sudo random numbers bro

fn swap<T:Copy>(list:&mut [T],x:usize,y:usize)
{
    let mut temp = list[x];
    list[x] = list[y];
    list[y] = temp;
}

// prints out the list of stuff in the collection
//
fn printlist<T:Display>(list:&[T])
{
    println!("collection:");
    for (i,item) in list.iter().enumerate()
    {
        println!("index: {} value: {}",i,item);
    }
}

fn isSorted<T:PartialOrd>(list:&[T]) -> bool
{
    for i in 1..list.len()
    {
        if list[i-1] > list[i]
        {
            return false; // rember, return is needed for early returns
        }
    }

    true
}

// bouble sort is best sort
fn boubleSort<T:PartialOrd+Copy+Display>(list:&mut [T])
{
    let mut sorted = false;

    while !sorted
    {
        sorted = true;
        for i in 1..list.len()
        {
            if list[i] < list[i-1]
            {
                println!("swapping!");
                sorted = false;
                swap(list,i,i-1);
            }
        }
    }
}

// add a merge sort 

fn main()
{
    let mut v1 = vec![1,2,3];
    swap(&mut v1,0,1);
    printlist(&v1);
    let mut v2 = vec![45,41,32,67,89,32,67,5,4,56,98,34,21];
    boubleSort(&mut v2[0..3]); // sort the first 4 elements
    printlist(&v2);
}
