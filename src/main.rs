fn print_vec(a: Vec<i32>){
    println!("{:?}", a);
}
fn print_vec_pointer(v: &Vec<i32>) {
    println!("{:p}", &v[0]);

}
fn main() {
   let mut a = vec![1, 2, 3 ,4, 5];
//    println!("size of vector on stack is {:?}", std::mem::size_of_val(&a));
//    let b = a.clone(); // ссылка на первый элемент и на количество элементов в нем 
//    println!("size of vector on heap is {:?}", std::mem::size_of_val(&b));
//    print_vec(b);
//    print_vec(a);
//    let (first_element, second_element )= a.split_at_mut(1);
//    println!("first element: {:?}", first_element[0]);
//    println!("second element: {:?}", second_element[0]);
//    let first =  &mut first_element[0];
//    let second = &mut second_element[3];
   print_vec_pointer(&a);
   a.extend(1..100);
   print_vec_pointer(&a);
//    *first +=  10;
//    *second += 20;

//    println!("{:?}", first);
//    println!("{:?}", second);
//    for x in &a {
//     println!("found elemnt of {:?}", x);
//    }

   print_vec(a);
}
