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
fn bouble_sort<T:PartialOrd+Copy+Display>(list:&mut [T])
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
fn selection_sort<T:PartialOrd+Copy+Display>(list:&mut [T])
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



fn merge<T:PartialOrd+Copy+Display+Clone>(list:&mut [T],pivot:usize)
{
    if isSorted(list) // assume that the input is sorted
    {
        // create temp arrays
        let left = &list[0..pivot];
        let right = &list[pivot .. list.len()];
        //printlist(left,"left");
        //printlist(right,"right");

        let mut index = 0;
        let mut temp_index = 0;

        let mut target = list.clone(); // we can't assing to a borrowed list, so we'll just clone it then assing to clone to the barrowed list, maybe
        

        // causes error: error[E0506]: cannot assign to `list[..]` because it is borrowed
        // .......................................................................................
        // //while index < list.len()
        // while temp_index < right.len() && temp_index < left.len()
        // {
        //     if left[temp_index] > right[temp_index] // left bigger than right
        //     {
        //         list[index] = right[temp_index];
        //         index = index + 1;
        //         temp_index = temp_index + 1;
        //         list[index] = left[temp_index];
        //     }
        //     else if left[temp_index] < right[temp_index] // right bigger than left
        //     {
        //         list[index] = left[temp_index];
        //         index = index + 1;
        //         temp_index = temp_index + 1;
        //         list[index] = right[temp_index];
        //     }
        //     else // left and right equal
        //     {
        //         list[index] = right[temp_index];
        //         index = index + 1;
        //         list[index] = left[temp_index];
        //
        //     }
        //
        //     index = index + 1;
        //     temp_index = temp_index + 1;
        // }

    }
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
    selection_sort(&mut data); // make sure the pass the vector as a mutable refrence
    printlist(&data,"after sort:");
    let piv = data.len() / 2;
    merge(&mut data,piv);
}
