fn main() {
    
    let mut a = [1,2,3,4];
    println!("values 1 is {}",a[0]);

    let b = [0;10];
    println!("values are {:?}",b);

    a[1] = 100;
    println!("values are {:?}",a);
}
