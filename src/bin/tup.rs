fn main(){
    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);

    //instead of x.clone(); we can use &x to print both x and y without generating the error produced when we assign x to y which is a move
}