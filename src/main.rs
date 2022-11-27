fn main() {
    println!("Hello, world!");

    let r = " "; // & = reference (stored in stack)
    // &str -> cannot be increased in size
    let str = String::from("hello"); // stored in heap because String is growable and it's size can be increased and is known at compile time only.
    /*
    
    hello-> stored in Heap
    pointer to str, its length (len=5) and capacity (cap=5) -> stored in stack
    
    */
    
    let int = 5; //this variable will be stored on stack because compiler knows the size of variable
    
    //Borrow Checker is the component that checks the ownership rules of our Rust code and ensures memory safety

    /*
    Ownership Rules:
    1) The variable is the owner of the value
    2) Every value can have a single owner at a time, i.e., a single variable cannot point to two different data at a time
    3) When owner goes out of scope, the compiler calls the drop method and the memory gets dropped (deleted)
    */

    {
        let s = String::from("abc");
    }
    // variable s is deleted from the memory outside of its scope, i.e.the brackets

    let a = 0;
    let b = a; //deep copy (copies the value of the variable)

    println!("{}", a);

    let s1 =  String::from("hello");
    let s2 = s1; // shallow copy (only copies the piinter, length and capacity of variable)
    // shallow copy leads to deletion of the variable that is being assigned, i.e., s1 will be deleted. This is known as 'move'
    println!("{:?}", s1); //Rust doesn't do deep copy of variables in the heap, therefore this statement is invalid as the ownership rule number 2 is getting violated
    //Deep copy is not done on heap because the operation of deep copy could be very expensive in terms of runtime performance

    //&str type -> borrowed string, String type -> owned string

    let ownedString = String::from("String");

    takes_ownership(ownedString);
    //passing ownership from ownedString to takes_ownership function
    println!("{:?}", ownedString);

    let mut myString = String::from("value");

    let mut myString2 = String::from("");

    let mut result = myString + myString2; // two owned strings are concatenated

    let result2 = "Value" + String::from("something"); // &str(borrowed string) + owned string
    
    let result3 = "Value" + "Something"; // &str(borrowed string) + &str(borrowed string)
    
    let result4 = String::from("Value") + "something"; // owned string + &str(borrowed string)

}

fn takes_ownership (s:String){
    println!("{:?}",s);
} // all data in this scope will be deleted due to passing of ownership