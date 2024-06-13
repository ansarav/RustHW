# RUST 


### VARIABLES
use i32 to start 
let" declares a variable
":" assigns the TYPE and "=" assigns the VALUE

"_" used to throw away the RETURN value of a function
> let _ = get_name();
> still need to use "let" tho


we can be more strategic with getting access to the value of tuples
> let(left,right) = slice.split_at(middle);

-----------------------------------------------------------------
### Declare a function and BLOCK:
> fn hi(){
> println!("Hello World"); // in Rust we need the "!" because of macro
> }

![image](https://github.com/ansarav/RustHW/assets/76548988/4a8276dd-3765-4740-88cd-6b773d85590f)
https://fasterthanli.me/articles/a-half-hour-to-learn-rust#blocks


We can also write statements as value set to a variable. aka BLOCKS
![image](https://github.com/ansarav/RustHW/assets/76548988/4a8276dd-3765-4740-88cd-6b773d85590f)
let x = {
    let y = 1; // first statement
    let z = 2; // second statement
    y + z // this is the *tail* - what the whole block will evaluate to
};

This limits the scope and means that y and  are not accesible outside of this block. So its like gatekeeping and
just doing its job that is was tasked to do inside that scope
// regardless we need a ";" at the end

-----------------------------------------------------------------
### IMPLICIT RETURN
the last expresion automatically becomes the return value thus no need to explicitly write "return"

-----------------------------------------------------------------

### ACCESS fields
DOT syntax to access portions of the value aka the fields of value
tuples: let pair = ("a",17) 
pair.0 // "a"
pair.1 // 17

:: syntax 
Crate -> Module -> Function
std::cmp::min(3,8)

"use" kind of like import the module
> use std::cmp::min
> let least = min(3,8); 

We can "use" byt using "{  }" to show like  a "list" of modules we are using rather than having to write a whole new use line
>ex: use std::cmp::{min.max};

Or if we just want to "use" all the modules/fields use "*" wildcard automatically imports all
>use std::cmp*;

You can also use a method in the sam eline you initialize a varible
> let x = "hola".len(); //4


-----------------------------------------------------------------
### Struct
Declare:

struct combo {
  x :i32,
  y: i32
}

initialize:
let first_price = combo{ x: 5, y =20}
// notice that when assigning a vlue for a filed in struct we use ":" not "="

lets say we want to make another varibble of type struct combo but you only want to cahnge 
hte value x you have from v2 and want ot keep the rest. you copie the values assigned by ".."
>
>  let v3 = Vec2 {
    x: 14.0,
    ..v2
 };





















