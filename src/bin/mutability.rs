fn main(){
    let s = String::from("hello, ");
    let mut s1 = s; //need to declare this as mutable so that it can be concatenated
    s1.push_str("world");
    println!("Success!");
    //the mutability doesn't carry forward after the deep copy.
    // i.e. if s is declared as mut, then s1 won't be mutable even if it deep copies s
}