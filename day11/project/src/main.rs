
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut smallest:&T = &list[0];

    for item in list{
        if item < smallest{
            smallest = item;
        }
    }

    smallest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = smallest(&number_list);
    println!("The smallest number is {result}");


}
