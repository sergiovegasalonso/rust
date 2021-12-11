

fn main() {
    
    // Variables
    let number_inmutable: u16 = 2;
    println!("number_inmutable: {}", number_inmutable);

    let mut number_mutable: u8 = 2;
    println!("number_mutable: {}", number_mutable);
    number_mutable = 8;
    println!("number_mutable: {}", number_mutable);

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let shadowing = 5;
    let shadowing = shadowing + 1;
    {
        let shadowing = shadowing * 2;
        println!("The value of x in the inner scope is: {}", shadowing);
    }
    println!("The value of x is: {}", shadowing);
    /*
    Shadowing is different from marking a variable as mut,
    because we’ll get a compile-time error if we accidentally try to reassign
    to this variable without using the let keyword.
    By using let, we can perform a few transformations on a value but have the
    variable be immutable after those transformations have been completed.
    */

    /*
    The other difference between mut and shadowing is that because
    we’re effectively creating a new variable when we use the let
    keyword again, we can change the type of the value but reuse
    the same name. For example, say our program asks a user to show how many 
    spaces they want between some text by inputting space characters,
    but we really want to store that input as a number:
    */
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    /*
    Compile error:
    let mut spaces = "   ";
    spaces = spaces.len();
    */


}
