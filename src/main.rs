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
fn printlist<T:Display>(list:&[T],msg:&str)
{
    println!("collection: {}",msg);
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
                //println!("swapping!");
                sorted = false;
                swap(list,i,i-1);
            }
        }
    }
}

// selection sort...
fn selectionSort<T:PartialOrd+Copy+Display>(list:&mut [T])
{
    for i in 0..list.len()
    {
        for j in (i+1)..list.len()
        {
            if list[i] > list[j]
            {
                swap(list,i,j);
            }
        }
    }
}


// function that can merge two slices in order
fn merge<T:PartialOrd+Copy+Display+Clone>(list1:&[T],list2:&[T]) -> Vec<T>
{
    let mut target:Vec<T> = Vec::new();

    if list1.len() == list2.len() // make sure the slices are the same length
    {
        let iter1 = list1.iter();
        let iter2 = list2.iter();

        loop
        {
            
        }
    }


    target
}

fn main()
{
    // let mut v2 = vec![45,41,32,67,89,32,67,5,4,56,98,34,21];
    // boubleSort(&mut v2[0..3]); // sort the first 4 elements
    // printlist(&v2,"after bouble sort...");
    // selectionSort(&mut v2[4..11]);
    // printlist(&v2[4..11],"after selection sort...");

    let mut data:Vec<i32> = Vec::new();
    for i in (0..10).rev()
    {
        data.push(i);
    }

    printlist(&data,"before sort:");
    selectionSort(&mut data); // make sure the pass the vector as a mutable refrence
    printlist(&data,"after sort:");
}
