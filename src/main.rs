fn main() {
    println!("Hello, world!");
    
    let a = (1,"Hello world",false);
    println!("The values are {} {}",a.0,a.1);

    let val3 = a.2;
    println!("Third value is {}",val3);

    //destructuring

    let (val1,val2,_) = a;
    println!("The values are {} {}",val1,val2);
}
