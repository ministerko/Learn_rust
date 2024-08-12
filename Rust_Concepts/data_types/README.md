# Data Types

two data type subsets: 
- scalar
- compound.

## scalar Types

Rust has four primary scalar types: 
- Integers
- characters 
- Booleans
- floating-point numbers

 #### Interger Types
 Interger can be signed or unsigned

  table shows built in data_types in rust  

 ```
 Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
```
![check codes->](./interger.rs)

You can write integer literals in any of the forms shown in Table

Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

```
Number literals	 Example
Decimal          98_222
Hex	             0xff
Octal	           0o77
Binary	         0b1111_0000
Byte (u8 only)	 b'A'
```
[interger overflow](https://medium.com/@mikecode/rust-integer-overflow-69277aad3ff5)

#### floting-Point Types

Rust floting-point types are **f32**
and **f64**

![check codes->](./floating_point.rs)

#### Numeric Operations
Rust supports the basic mathematical operations you’d expect for all the number types,those operations are 

//addition
- sum (+)

//subtraction
- difference (-)

//multiplication
- product (*)

//division
- quotient
- truncated

//remainder
- remainder (%)
  
![check codes->](./numeric_operation.rs)


#### Boolean Type
a Boolean type in Rust has two possible values: **true** and **false.** Booleans are one byte in size. The Boolean type in Rust is specified using bool

![check codes->](./boolean_type.rs)

#### Character Type

Rust’s char type is the language’s most primitive alphabetic type

![check codes->](./character.rs)

## Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types

- tuples
- arrays

#### The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type

![check codes->](./tuple_Type.rs)

#### The Array Type

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.

example:

```
fn main(){
  let a = [1,2,3,4,5];

}
```

**where array is useful ??**
- Arrays are useful when you want your data allocated on the stack rather than the heap
-  when you want to ensure you always have a fixed number of elements
  example:
  ```
  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
  ```


*note:*
- An array isn’t as flexible as the vector type
- A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
- If you’re unsure whether to use an    array or a vector, chances are you should use a vector

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```
let a: [i32; 5] = [1, 2, 3, 4, 5];

```

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```
let a =[3; 5];

//same as writing 

//let a = [3,3,3,3,3];
```
**Accessing Array Elements**
An array is a single chunk of memory of a known, fixed size that can be allocated on the stack

![check codes->](./Array_type.rs)
