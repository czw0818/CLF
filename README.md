# The CLF ***toy-***language

this is a Multi paradigm toy programming language
The `function` is a first-class citizen

- notice
- control flow
- cyclic structure
- function
- composite structure
- trait
- patten match
- grammar sugar
- use the module
- magic

## Notice

`main`,`print` and `println` are keywords:
```rust
main{
    println("hello world")
}
```
so you also can:

```rust
main{
    "hello world".println
    println << "hello world"
}
```

other types are **all**  `!`'s subtype

## Control-flow

control flow looks like *Rust*:
```rust
if (condintion){
    /*  something  */
}else if (condition){
    // something
}else{
    // something
}
```
Please do **not** omit parentheses, which will cause **syntax error** because of some *grammar sugar*

## Cyclic-Structure

`while` is the most important cyclic struct
```rust
while (condition){
    // something
}
``` 

`for` is a independence structure **by now**.
It is **planned** to be made into grammar sugar
for syntax:
```rust
for (i in 0..11){ // Parentheses cannot be omitted
    println(i)
}
```
of course,We provide automatic deconstruction,like this:
```rust
for ((x,y) in [(0,0),(1,1),(2,4),(3,9)]){
    println("the x is {},the y is {}",x,y)
}
```

We plan to expand for to this(**Experimental**):
```rust
var tmp = 0..9;
while (let Some(i) = temp){
    println(i)
}
```
### notice in cyclic
`break`,`continue`,`return` are expression,their type is `!`
e.g.:
```rust
let a: int
let b: str

for (i in 1..9) {
    if (i==3){
        a = continue // return `!` type
    }
    if (i==5){
        b = break // return `!` type
    }
}
```
## Function
Although `main` is a keyword and does't need keyword `fn` to declare,but `main()` still is a function.
notice:`main()` may **not** be the first function.See chapter magic for details

function is a first-class citizen.it looks like this:
```rust
fn fib(n:int) -> int{
    if (n <= 1){
        return n 
    }
    return fib(n-1)+fib(n-2)
}
```
We require that the return value must be returned using the `return` statement
If there are no parameters, the parameter list and even **parentheses** can be omitted
If the return value is unit, it can be omitted
e.g.:
```rust
fn say{
    println("helloworld")
}
```
### closure
e.g:
```rust
let mul_by_two:fn (int) -> int = {x => x*2}
assert_eq!(mul_by_two( 2 ) , 2*2 )
```
Or multi statement without parameters:
```rust
let exmaple : fn () = {
    3%5 1+10 print("hello world")
}
example()
```
When there is a non unit return value, the return statement must be used in the closure

## composite structure

We provides three composite structure.`struct`,`union` and `enum`

### Struct

```rust
struct Person {
    name:str,age:int
}

impl Person{
    fn new(name:str,age:int) -> Self{
        Self{name:name,age:age}
    }
    fn get_age(self) -> int{ // self is a ref
        self.age
    }
    fn set_age(mut self,age:int) {
        self.age = age
    }
}

main{
    let me = Person::new("my name",0)
    println(me.get_age()) // display:0
    // println(me.set_age()) Error:`me` is immut
}
```
Of course, we support generics:
```rust
struct Value<T>(T)
impl<T> Value<T>{
    fn new(v:T) -> Self{
        Self(v)
    }
}
```
Or:
```rust
type T:Display{// T impl `Display`,Generics continue to be valid in the block
    struct Value(T)
    impl Value{
        fn new(v:T) -> Self{
            Self(v)
        }
    }
}
```
### Union(unsafe+Experimental)
I will not introduce `union` until it is stable

### Enum(Unstable)
```rust
#[derive(PartiEq)]
enum Order{
    Greater(int),
    Eq(bool),
    Less(str)
}

impl Order{
    fn is_greater(self) -> bool{
        //Ignore magic and patten first, and it will be introduced later
        matches!(self , Order::Greater(_))
    }
}
```

## Trait

This concept is like the interface of other languages
But trait is not a type.
Trait can choose to provide a default implementation of certain methods
such as,traits:`Into`,`From`,`PartiEq`
If you implementation Into<T> for U,compiler will auto implement From<U> for T 

## Patten match

```rust
if let Ok(v) = Ok(1){
    println(v) // it will display `1`
}
```

```rust
match Option::Some("hello world"){
    Some(v) => println("I get the `{}`",v),
    None => println("There is nothing")
}
```

## Grammar sugar

If the last (or only) parameter of the function is a closure, we can put the closure outside and close to the function

e.g.:
```rust
let some_num = Option<int>::None
some_num.get_or_else(){
    println("closure was execed")
    return 100
}.println
```
This is also why branch and loop structures must be bracketed.

`_` is a keyword for some grammar sugar
e.g.:
```rust
let add_two = _ + 2
assert!(add_two(2),2+2)
```
same as:
```rust
let add_two = {x => x + 2}
assert!(add_two(2),2+2)
```

Use the: > operator to throw the value on the left to the function on the right for parameter calling. You can nest it, and you have this magical thing:
```rust
let value = 4 :> _ + 2 :> println // 显示6
assert_val_type!( value , () )
```
TODO!()

### use the module
TODO!()

### magic
Magic is similar to rust's process macro
```rust
magic! assert{
    // LParent,RParent... are auto provided when compile
    var cpl = Compiler! // Compile time implementation Compiler!
    cpl.eat_if_is::<LParent>()
    let condition = cpl.parse::<Expression>().unwrap() 
    // Throw Compiler Error if panic
    cpl.eat_if_is::<RParent>()
    cpl.insert(Ast!{
        if (! $condition){
            panic!("assert failed")
        }
    })
}
```