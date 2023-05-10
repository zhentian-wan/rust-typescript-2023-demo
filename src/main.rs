fn main() {
    let _nums: Vec<_> = vec![1,2,3]
        .iter() // create the iterator to go over the elements in the array
        .map(|x| x + 1) // do the operation
        .collect(); // convert back to a vector
    println!("{:?}", _nums)
}
