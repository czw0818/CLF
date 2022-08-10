# CLF 玩具语言(this `.md` file is provides for chinese)

这是一个多范式的玩具编程语言
`function` 是一等公民

- 注意事项
- 控制流
- 循环结构
- 函数
- 复合结构
- 特征(trait)
- 模式匹配
- 语法糖
- 使用模块
- 魔法(元编程)

## 注意事项

`main`,`print` 还有 `println` 都被视为关键字:
```rust
main{
    println("你好世界")
}
```
所以理论上你可以这样写:

```rust
main{
    "你好世界".println
    println << "你好世界"
}
```

**所有**类型都是 `!` 的子类型

## 控制流

控制流语法类似`Rust`:
```rust
if (condintion){
    /*  辣鸡代码  */
}else if (condition){
    // 没用代码
}else{
    // 摸鱼代码
}
```
请**不要**去掉小括号, 因为某些语法糖......为了预防二义性，拼了

## 循环结构

`while` 是这个语言最喜欢的循环结构
```rust
while (condition){
    // 某些代码
}
``` 

`for` **目前为止**还是独立的结构.但它被**计划**设计成语法糖。别问,问就是喜欢`while`
来段代码看看吧:
```rust
for (i in 0..11){ // 小括号不能去
    println(i)
}
```
借鉴于Rust,允许自动解构元组:
```rust
for ((x,y) in [(0,0),(1,1),(2,4),(3,9)]){
    println("the x is {},the y is {}",x,y)
}
```

我打算把`for`搞成这副鬼样子(**不稳定的**):
```rust
var tmp = 0..9;
while (let Some(i) = temp){
    println(i)
}
```
### notice in cyclic
方便起见，我们把`break`,`continue`,`return` 设计了类型为`!`的表达式
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
因为`main`是关键字,所以声明`main`函数不需要`fn`关键字
敲黑板:受某些魔法影响,`main()` 可能**不是**第一个被调用的函数.

函数,一等公民,就这样:
```rust
fn fib(n:int) -> int{
    if (n <= 1){
        return n 
    }
    return fib(n-1)+fib(n-2)
}
```
我们要求明确使用`return`返回值。但是如果没有参数，整个列表连带**括号**都不用写.
类似`Rust`的，如果返回一个`unit`,我都懒得写
e.g.:
```rust
fn say{
    println("某某")
}
```
### 闭包
e.g:
```rust
let mul_by_two:fn (int) -> int = {x => x*2}
assert_eq!(mul_by_two( 2 ) , 2*2 )
```
无参多语句长这样:
```rust
let exmaple : fn () = {
    3%5 1+10 print("一点东西")
}
example()
```

如果没有`return`,那自动返回`()`也就是`unit`
## 复合类型

三个复合类型够摆没.`struct`,`union`,`enum`

### Struct

```rust
struct Person {
    name:str,age:int
}

impl Person{
    fn new(name:str,age:int) -> Self{
        Self{name:name,age:age}
    }
    fn get_age(self) -> int{ // self是个引用,GC语言嘛，不用Rust那么麻烦
        self.age
    }
    fn set_age(mut self,age:int) {// 可变引用
        self.age = age
    }
}

main{
    let me = Person::new("my name",0)
    println(me.get_age()) // 显示个0
    // println(me.set_age()) Error:`me·不可变啊
}
```
泛型支持:
```rust
struct Value<T>(T)
impl<T> Value<T>{
    fn new(v:T) -> Self{
        Self(v)
    }
}
```
真的创新创死我了，你就假装很喜欢这个语法怎么样:
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
### Union(不安全不稳定不卫生)
没个一年两年的不介绍这破玩意了

### Enum(不稳定的语言之子)
```rust
#[derive(PartiEq)]
enum Order{
    Greater(int),
    Eq(bool),
    Less(str)
}

impl Order{
    fn is_greater(self) -> bool{
        //看不懂别看了，后面会讲的
        matches!(self , Order::Greater(_))
    }
}
```

## 特征

这破玩意挺像接口`interface`的，但`trait`不是一个类型,当然你可以用`impl trait`这个泛型的语
法糖。`trait`允许你提供默认实现.详见:`Into`,`From`,`PartiEq`
顺便说下,你如果实现了 Into<T> for U,编译器会帮你实现 From<U> for T，反之亦然 

## 模式匹配

```rust
if let Ok(v) = Ok(1){
    println(v) // 显示 `1`
}
```

```rust
match Option::Some("全球高考"){
    Some(v) => println("我拿到了!!!《{}》",v),
    None => println("没抢到")
}
```

## Grammar sugar

### 关于闭包
如果函数(闭包)最后(或是唯一)一个参数是给函数,你可以直接在外面以闭包形式实现
就像这样:

e.g.:
```rust
let some_num = Option<int>::None
some_num.get_or_else(){
    println("闭包执行了")
    return 100
}.println
```
这也是为什么某些结构不能删括号

下划线是个关键字:(想不到吧
e.g.:
```rust
let add_two = _ + 2
assert!(add_two(2),2+2)
```
这等同于:
```rust
let add_two = {x => x + 2}
assert!(add_two(2),2+2)
```

### 关于复合函数

使用 :>运算符可以把左边的值丢给右边的函数做参数调用,可以嵌套,就有了这神奇的
玩意:
```rust
let value = 4 :> _ + 2 :> println // 显示6
assert_val_type!( value , () )
```

### 使用模块
TODO!()

### magic
魔法，更像Rust的过程宏的加强版,加强在可读性上
```rust
magic! assert{
    // LParent,RParent...这些类型编译器会给你提供
    var cpl = Compiler! // 编译器会给你实现Compiler!
    cpl.eat_if_is::<LParent>()
    let condition = cpl.parse::<Expression>().unwrap() 
    // 如果panic!()了，那就编译错误
    cpl.eat_if_is::<RParent>()
    cpl.insert(Ast!{
        if (! $condition){
            panic!("assert failed")
        }
    })
}
```