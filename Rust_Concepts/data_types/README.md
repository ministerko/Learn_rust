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

 ### Interger Types
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
![check this->](./interger.rs)

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