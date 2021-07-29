# Rust学习之路

## Rust输出到命令行

* Rust输出到文字的方式主要有两种: `pirntln!()` 和 `print!()`

    这两个“函数”都是命令行输出字符串的方法，区别仅在于前者会在输出最后附加一个换行符。当用这两个“函数”输出信息时，第一个参数时格式字符串，后面是一串可变参数，对应着格式字符串重的“占位符”，这一点与C语言中的printf函数很相似。但是，Rust中格式字符串中的占位符不是“%+字母”的形式，而是一对{}

> 举例

```rust
fn mian(){
    let a = 12;
    println!("a si {}", a);
}
```

以上程序的输出结果是：
> a is 12

如果我想把a输出两遍，不要写成：

```rust
println!("a is {},a again is {}", a, a);
```

更高的写法是：

```rust
println!("a is {0},a again is {0}", a, a);
```

在 `{}` 之间可以放一个数字，它将把之后的可变参数当作一个数组来访问，下标从0开始。
如果要输出 `{` 或 `}` 怎么办呢？格式字符串中通过 `{{` 和 `}}` 分别转义代表 `{` 和 `}` 。但是其他转义字符与C语言里的转义字符一样，都是反斜杆\开头的形式。

```rust
fn main(){
    println!("{{}}");
}
```

以上程序的输出结果是：
>{}

## Rust基础语法

### 变量

首先必须说明，Rust是强类型语言，但具有自动判断变量类型的能力。  
如果要声明变量，需要使用 `let` 关键字。例如：

```rust
let a = 123;
```

在这句声明语句之后，以下三行代码都是被禁止的：

```rust
a = "abc";
a = 4.56;
a = 456;
```

第一行的错误在于当声明a是123以后，a就被确定为整型数字，不能把字符串类型的值赋给它。  
第二行的错误在于自动转换数字精度有损失，Rust语言不允许精度有损失的自动数据类型转换。  
第三行的错误在于a不是个可变变量。

> 第三行代码解释  
Rust语言为了高并发安全而做的设计：在语言层面尽量少的让变量的值可以改变。所以a的值不可变。官方文档称a这种变量为“不可变变量”。

如果我们编写的程序的一部分在假设值永远不会改变的情况下运行，而我们代码的另一个部分在改变该值，那么代码不改变值的那一部分可能就不会按照设计的意图去运转。由于这种原因造成的错误很难在事后找到。这是Rust语言设计这种机制的原因。  
使变量变得“可变”（mutable）只需要一个mut关键字。

```rust
let mut a = 123;
a = 456;
```

这个程序是正确的。

### 常量与不可变变量的区别

在Rust中，以下程序是合法的：

```rust
let a = 123;
let b = 456;
```

但是如果a是常量就不合法;

```rust
const a: i32 = 123;
let  a = 456;
```

变量的值可以重影，但在重影以前不能私自被改变，这样可以确保在每一次重影之后的区域编译器可以充分的推理程序逻辑。虽然Rust有自动判断类型的功能，但有些情况下声明类型更加方便：

```rust
let a: u64 = 123;
```

这里声明了a为无符号64位整型变量，如果没有声明类型，a将自动被判断位有符号32位整型变量，这对于a的取值范围有很大的影响。

### 重影

重影的概念与其他面向对象语言里的“重写”（Override）或“重载”（Overload）是不一样的。  
重影就是指变量的名称可以被重新使用的机制：

```rust
fn main(){
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

这段程序的运行结果：

> The value of x is: 12

重影与可变变量的赋值不是一个概念，重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。但可变变量赋值仅能发生值的变化。

```rust
let mut s = "123";
s = s.len();
```

这段程序会出错：不能给字符串变量赋整型变量。

## Rust数据类型

### 整数型（Integer）

整数型简称整型，按照比特位长度和有无符号分为以下种类：

|位长度|有符号|无符号|
|--|--|--|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

**isize** 和 **usize** 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标平台，如果是32位架构的处理器将使用32位位长度整型。

整数的表述方法有以下几种：

|进制|例子|
|--|--|
|十进制|98_222|
|十六进制|0xff|
|八进制|0o77|
|二进制|0b1111_0000|
|字节（只能表示u8型）|b'A'|

有的整数中间存在一个下划线，这种设计可以让人们在输入一个很大的数字时更容易判断数字的值大概是多少。

### 浮点数型（Floating-Point）

Rust与其它语言一样支持32位浮点数（f32）和64位浮点数（f64）。默认情况下，64.0将表示64位浮点数，因为现代计算机处理器对于两种浮点数计算的速度几乎相同，但64位浮点数精度更高。

```rust
fn main(){
    let x = 2.0; //f64
    let y: f32 = 3.0; //f32
}
```

### 数学运算

用一段程序反应数学计算：

```rust
fn main(){
    let sum = 5 + 10; //加
    let difference = 95.5 - 4.3; //减
    let product = 4 * 30; //乘
    let quotient = 56.7 / 32.2 //除
    let remainder = 43 % 5; //求余
}
```

许多运算符号之后加上=号是自运算的意思，例如：  
`sum += 1`等同于`sum = sum + 1`  
**注意：** Rust不支持++和--，因为这两个运算符出现在变量的前后会影响代码可读性，减弱了开发者对变量改变的意识能力。

### 布尔型

布尔型用bool表示，值只能为true或false

### 字符型

字符型用char表示。  
Rust的char类型大小为4字节，代表Unicode标量值。中文，日文和韩文字等非英文字符甚至表情符号和零宽度空格在Rust中都是有效的char值。  
Unicode值的范围从U+0000到U+D7FF和U+E000到U+10FFFF（包括两端），但是，“字符”这个概念并不存在于Unicode中，因此对“字符”是什么的直觉可能与Rust中的字符概念不匹配。所以一般推荐使用字符串储存UTF-8文字（非英文字符尽可能地出现在字符串中）。  
**注意：** 由于中文文字编码有两种（GBK和UTF-8），所以编程中使用中文字符串可能导致乱码的出现，这是因为源程序与命名行的文字编码不一致，所以在Rust中字符串和字符都必须使用UTF-8编码，否则编译器会报错。

### 复合类型

元组用一对`()`包括的一组数据，可以包含不同种类的数据：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
/*  tup.0 等于 500
    tup.1 等于 6.4
    tup.2 等于 1
 */
let (x, y, z) = tup;
// y 等于 6.4
```

数组用一对`[]`包括的同类型数据。

```rust
let a = [1, 2, 3, 4, 5];
// a是一个长度为5的整型数组

let b = ["January", "February", "March"];
// b是一个长度为3的字符串数组

let c: [i32; 5] = [1, 2, 3, 4, 5];
// c是一个长度为5的i32数组

let d = [3; 5];
// 等同于let d = [3, 3, 3, 3, 3];

let first = a[0];
let second = a[1];
// 数组访问

a[0] = 123; // 错误：数组a不可变
let mut a = [1, 2, 3];
a[0] = 4; // 正确
```

## Rust注释

Rust中的注释方式与其它语言（C、Java）一样，支持两种注释方式：

```rust
// 这是第一种注释方式

/* 这是第二种注释方式 */

/*
 * 多行注释
 * 多行注释
 * 多行注释
 */
```

## 用于说明文档的注释

在Rust中使用`//`可以使其之后到第一个换行符的内容变成注释。  
在这种规则下，三个反斜杠`///`依然是合法的注释开始。所以Rust可以用`///`作为说明文档注释的开头：

```rust
/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let x = add(1, 2);
/// 
/// ```

fn add(a: i32,b: i32) -> i32 {
    return a + b;
}

fn main(){
    println！("{}",add(2,3));
}
```

运行`cargo doc`会生成这个文档注释的HTML文档。  

![HTML文档](README_files/1.jpg)

运行`cargo doc --open`会构建当前crate文档的HTML并在浏览器中打开。

![HTML浏览器](README_files/2.jpg)

运行`cargo test`将会对注释文档中的示例代码进行测试

![测试](README_files/3.jpg)

## Rust函数

函数在Rust语言中是普遍存在的。

> `fn <函数名>(<参数>){<函数体>}`

其中Rust函数名称的命名风格是小写字母以下划线分割：

```rust
fn main(){
    println!("Hello, world!");
    another_function();
}

fn another_function(){
    println!("Hello, runzm!");
}
```

运行结果：
> Hello,world!  
Hello,runzm!

**注意：** 在源代码中的main函数之后定义了another_function。Rust不在乎在何处定义函数，只需在某个地方定义他们即可。

### 函数参数

Rust中定义函数如果需要具备参数必须声明参数名称和类型：

```rust
fn main(){
    anther_function(5, 6);
}

fn anther_function(x: i32, y: i32){
    println!("x的值为：{}", x);
    println!("y的值为：{}", y);
}
```

### 函数体的语句和表达式

Rust函数体由一系列可以以表达式（Expression）结尾的语句（Statement）组成。到目前为止，我们仅见到了没有以表达式结尾的函数，但已经将表达式用作语句的一部分。  
语句是执行某些操作且没有返回值的步骤。例如：  

```rust
let a = 6
```

这个步骤没有返回值，所以以下语句不正确：

```rust
let a = (let b = 2);
```

表达式有计算步骤且有返回值。以下是表达式（假设出现的标识符已经被定义）：

```rust
a = 7
b + 2
c * (a + b)
```

Rust中可以在一个用`{}`包括的块里面编写一个较为复杂的表达式：

```rust
fn main(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x的值为：{}", x);
    println!("y的值为：{}", y);
}
```

运行结果：
> x的值为：5  
y的值为：4

这段程序中包含了一个表达式块：

```rust
{
    let x = 3;
    x + 1
};
```

而且在块中可以使用函数语句，最后一个步骤是表达式，此表达式的结果值是整个表达式块所代表的值。这种表达式块叫做函数体表达式。  
**注意：** `x + 1`之后没有分号，否则它将变成一条语句！  
这种表达式块是一个合法的函数体。而且在Rust中，函数定义可以嵌套：

```rust
fn main(){
    fn five() -> i32{
        5
    }
    println!("five()的值为：{}",five());
}
```

### 函数返回值

在上一个嵌套的例子中已经显示了Rust函数声明返回值类型的方式：在参数声明之后用`->`来声明函数返回值的类型（不是`:`）。  
在函数体中，随时都可以以return关键字结束函数运行并返回一个类型合适的值。这也是最接近大多数开发者经验的做法：

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

但是Rust不支持自动返回值类型判断！如果没有明确声明函数返回值的类型，函数将被认为是“纯过程”，不允许产生返回值，`return`后面不能有返回值表达式。这样做的目的是为了让公开的函数能够形成可见的公报。  
**注意：** 函数表达式并不能等同于函数体，它不能使用 **return关键字**。

## Rust条件语句

在Rust语言中的条件语句是这种格式的：

```rust
fn main(){
    let number = 3;
    if number < 5 {
        println!("条件为true");
    } else {
        println!("条件为false");
    }
}
```

在上述程序中有条件`if`语句，这个语法在很多其它语言中很常见，但也有一些区别：首先，条件表达式`number < 5`不需要用小括号包括（**注意：**不需要不是不允许）。但是Rust中的if不存在单语句不用加`{}`的规则，不允许使用一个语句代替一个块。尽管如此，Rust还是支传统else-if语法的：

```rust
fn main(){
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    }
    else if a < 0 {
        b = -1;
    }
    else {
        b = 0;
    }
    println!("b is {}", b);
}
```

运行结果：
> b is 1

Rust中的条件表达式必须是bool类型，例如下面的程序就是错误的：

```rust
fn main(){
    let number = 3;
    if number{  // 报错，expected `bool`, found intergerrustc(E0308)
        println!("Yes");
    }
}
```

虽然C/C++语言中的条件表达式用整数表示，非0即真，但这个规则在很多注重代码安全性的语言中是被禁止的。
结合之前章学习的函数体表达式，我们加以联想：

```rust
if <condition> {block 1} else {block 2}
```

这种语法中的`{block 1}`和`{block 2}`可不可以是函数体表达式呢？  
答案是肯定的！在Rust中我们可以使用`if-else`结构实现类似于三元条件运算表达式`(A?B:C)`的效果：

```rust
fn main(){
    let a =3;
    let number = if a > 0 { 1 } else { 0 };
    println!("number 为 {}", number);
}
```

运行结果：
> number 为 1

**注意：**两个函数体表达式的类型必须一样！且必须有一个else及其后面的表达式块。

## Rust循环

### while循环

while循环是最典型的条件语句循环：

```rust
fn main(){
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");
}
```

运行结果：
> 1  
2  
3  
EXIT

Rust语言到今日还没有do-while的用法，但是do被规定为保留字，也许以后的版本中会用到。  
在C语言中for循环使用三元语句控制循环，但是Rust中没有这种用法，需要用while循环来替代：

> c语言

```c
int i;
for (i = 0; i < 10; i++){
    // 循环体
}
```

> Rust

```rust
let mut i = 0;
while i < 10 {
    // 循环体
    i += 1;
}
```

### for循环

for循环是最常用的循环结构，常用来遍历一个线性数据结构（比如数组）。for循环遍历数组：

```rust
fn main(){
    let a = [10, 20, 30, 40, 50];
    for i in a.iter(){
        println!("值为：{}", i);
    }
}
```

运行结果：

> 值为：10  
值为：20  
值为：30  
值为：40  
值为：50

这个程序中的`for`循环完成了对数组a的遍历。`a.iter()`代表`a`的迭代器（iterator）  
for循环其实是可以通过下标来访问数组的：

```rust
fn main(){
    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{0}] = {1}", i, a[i]);
    }
}
```

运行结果：
> a[0] = 10  
a[1] = 20  
a[2] = 30  
a[3] = 40  
a[4] = 50

### loop循环

经常碰到这样的情况：某个循环无法在开头和结尾判断是否继续进行循环，必须在循环体中间某处控制循环的进行。如果遇到这种情况，一般在一个`while(true)`循环体里实现中途退出循环的操作。  
Rust语言有原生的无限循环结构——loop：

```rust
fn main(){
    let s = ['R', 'U', 'N', 'Z', 'M'];
    let mut i = 0;
    loop{
        let ch = s[i];
        if ch == 'Z' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }    
}
```

运行结果：
> 'R'  
'U'  
'N'

loop循环可以通过break关键字类似于return一样使整个循环退出并给予外部一个返回值。这是一个十分巧妙的设计，因为loop这样的循环常被用来当做查找工具使用，如果找了某个东西当然要将这个结果交出去：

```rust
fn main(){
    let s = ['R', 'U', 'N', 'Z', 'M'];
    let mut i = 0;
    let location = loop{
        let ch = s[i];
        if ch == 'Z' {
            break i;
        }
        i += 1;
    };
    println!("\'Z\' 的索引为 {}", location);
}
```

运行结果：
> 'Z' 的索引为 3

## Rust所有权

计算机程序必须在运行时管理它们所使用的内存资源。  
大多数的编程语言都有管理内存的功能：  
C/C++这样的语言主要通过手动方式管理内存，开发者需要手动的申请和释放内存资源。但为了提高开发效率，只要不影响程序功能的实现，许多开发者没有及时释放内存的习惯。所以手动管理内存的方式通常造成资源浪费。  
Java语言编写的程序在虚拟机（JVM）中运行，JVM具备自动回收内存资源的功能。但这种方式常常会降低运行时效率，所以JVM会尽可能少的回收资源，这样也会使程序占用较大的内存资源。
所有权是一个新颖的概念，它是Rust语言为高效使用内存而设计的语法机制。所有权概念是为了让Rust在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念。

### 所有权规则

所有权有以下三条规则：

* Rust中的每个值都有一个变量，称为其所有者。

* 一次只能有一个所有者。

* 当所有者不在程序运行范围时，该值将被删除。

这三条规则是所有权概念的基础。  
接下来将介绍与所有权概念有关的概念。

### 变量范围

用下面这段程序描述变量范围的概念：

```rust
{
    // 在声明以前，变量s无效
    let s ="runzm";
    // 这里是变量s的可用范围
}
// 变量范围已经结束，变量s无效
```

变量范围是变量的一个属性，其代表变量的可行域，默认从声明变量开始有效直到变量所在域结束。

### 内存与分配

如果我们定义了一个变量并给它赋予一个值，这个变量的值存在于内存中。这种情况很普遍。但如果我们需要储存的数据长度不确定（比如用户输入的一串字符串），我们就无法在定义时明确数据长度，也就无法在编译阶段令程序分配固定长度的内存空间供数据存储使用。这就需要提供一种在程序运行时程序自己申请使用内存的机制——堆。这里的"内存资源"都指的是堆所占用的内存空间。  
有分配就有释放，程序不能一直占用某个内存资源。因此决定资源是否浪费的关键因素就是资源有没有及时的释放。  
字符串样例程序用C语言等价编写：

```c
{
    char *s = strdup("runzm");
    free(s); // 释放s资源
}
```

显然，Rust中没有调用free函数来释放字符串s的资源（这里在C语言中是不正确的写法，因为“runoob”不在堆中，这里假设它在）。Rust之所以没有明示释放的步骤是因为在变量范围结束的时候，Rust编译器自动添加了调用释放资源函数的步骤。  
这种机制看似很简单了：它不过是帮助程序员在适当的地方添加了一个释放资源的函数调用而已。但这种简单的机制可以有效地解决一个史上最令程序员头疼的编程问题。

### 变量与数据交互的方式

变量与数据交互的方式主要有移动（Move）和克隆（Clone）两种：

### 移动

多个变量可以在Rust中以不同的方式与相同的数据交互：

```rust
let x = 5;
let y = x;
```

这个程序将值5绑定到变量x，然后将x的值复制并赋值给变量y。现在栈中将有两个值5。此情况中的数据是“基本数据”类型的数据，不需要储存到堆中，仅在栈中的数据的“移动”方式是直接复制，这不会花费更长的时间或更多的储存空间。“基本数据”类型有这些：

* 所有整数类型，例如i32、u32、i64等。

* 布尔类型bool，值为true或false。

* 所有浮点类型，f32和f64。

* 字符类型char。

* 仅包含以上类型数据的元组（Tuples）。

但如果发生交互的数据在堆中就是另外一种情况：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

第一步产生一个S听对象，值为“hello”。其中“hello”可以认为是类似于长度不确定的数据，需要在堆中储存。  
第二步的情况略有不同（ **这不是完全真的，仅用来对比参考** ）：

![移动1](README_files/4.jpg)

如图所示：两个String对象在栈中，每个String对象都有一个指针指向堆中的“hello”字符串。在给s2赋值时，只有栈中的数据被复制了，堆中的字符串还是原来的字符串。  
当变量超出范围时，Rust自动调用释放资源函数并清理该变量的堆内存。但是s1和s2都被释放的话，堆区中的“hello”将被释放两次，这是不被系统允许的。为了确保安全，在给s2赋值时s1已经无效了。没错，在把s1的值赋给s2以后s1将不可以再被使用。下面这段程序是错的：

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1); //错误！s1已经失效
```

所以实际情况是：

![移动2](README_files/5.jpg)

s1名存实亡。

### 克隆

Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，且采用移动的方式进行数据交互。但如果需要将数据单独的复制一份以供他用，可以使用数据的第二种交互方式——克隆。

```rust
fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

运行结果：
> s1 = hello, s2 = hello

这里是真的将堆中的“hello”复制了一份，所以s1和s2都分别绑定了一个值，释放的时候也会被当作两个资源。  
克隆仅在需要复制的情况下使用，毕竟复制数据会花费更多的时间。

### 涉及函数传参的所有权机制

对于变量来说这是最复杂的情况。  
如果将一个变量当做函数的参数传给其他函数，对于怎样安全的处理所有权？
下面这段程序描述了这种情况下所有权机制的运行原理：

```rust
fn main(){
    let s = String::from("hello");
    // s被声明有效

    takes_ownership(s);
    // s的值被当作参数传入函数
    // 所以可以当作s已经被移动，从这里开始已经无效

    let x = 5;
    // x被声明有效

    makes_copy(x);
    // x的值被当作参数传入函数 
    // 但x 是基本类型，依然有效
    // 在这里依然可以使用x却不能使用s

} // 函数结束，x无效，然后是s，因为s已被移动，所以不用被释放

fn takes_ownership(some_string: String){
    // 一个String参数some_string传入，有效
    println!("{}", some_string);
}// 函数结束，参数some_string在这里释放

fn makes_copy(some_integer: i32){
    // 一个i32参数some_integer传入，有效
    println!("{}", some_integer)
}// 函数结束，参数some_integer是基本类型，无需释放
```

如果将变量当做参数传入函数，那么它和移动的效果是一样的。

### 函数返回值的所有权机制

```rust
fn main(){
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2被当作参数移动，s3获得返回值所有权
}// s3无效被释放，s2被移动，s1无效被释放

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string被声明有效

    return some_string;
    // some_string被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效
    a_string // a_string 被当作返回值移出函数
}
```

被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放。

### 引用与租借

引用（Reference）是C++开发者较为熟悉的概念。  
可以把它看作一种指针。  
实质上“引用”是变量的间接访问方式。

```rust
fn main(){
    let s1 = String::from("hello");
    let s2 =  &s1;
    println!("s1 is {}, s2 is {}", s1, s2);
}
```

运行结果：
> s1 is hello, s2 is hello

`&`运算符可以取变量的“引用”。  
当一个变量的值被引用时，变量本身不会被认定无效。因为“引用”并没有在栈中复制变量的值：  

![引用](README_files/6.jpg)

函数参数传递的道理一样：

```rust
fn main(){
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}
```

运行结果：
> The length of 'hello' is 5.

引用不会获得值的所有权。  
引用只能租借（Borrow）值的所有权。  
引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：

```rust
fn main(){
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = s1;
    println!("{}", s2);
}
```

这段程序不正确：因为s2租借的s1已经将所有权移动到s3，所以s2将无法继续租借使用s1的所有权。如果需要使用s2该值，必须重新租借：

```rust
fn main(){
    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; //重新从s3租借所有权
    println!("{}", s2);
}
```

这段程序是正确的。  
既然引用不具有所有权，即使它租借了所有权，它也只享有使用权（这跟租房子是一个道理）。  
如果尝试利用租借来的权利来修改数据会被阻止：

```rust
fn main(){
    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    s2.push_str("zm"); // 错误，禁止修改租借的值
    println!("{}", s2);
}
```

这段程序中s2尝试修改s1的值被阻止，租借的所有权不能修改所有者的值。  
当然，也存在一种可变的租借方式，就像租一个房子，如果物业规定房主可以修改房子结构，房主在租借时也在合同中声明赋予这种权利，那么是可以重新装修房子的：

```rust
fn main(){
    let mut s1 = String::from("run");
    // s1 是可变的
    let s2 = &mut s1;
    // s2 是可变的引用

    s2.push_str("zm");
    println!("{}", s2);
}
```

这段程序就没有问题了。用`&mut`修饰可变的引用类型。  
可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以：

```rust
fn main(){
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; //错误，可变引用不允许多重引用

    println!("{}, {}", r1, r2)
}
```

这段程序不正确，因为多重可变引用了s。  
Rust对可变引用的这种设计主要出于对并发状态下发送数据访问碰撞的考虑，在编译阶段就避免了这种事情的发送。  
由于发生数据访问碰撞的必要条件之一就是数据被至少一个使用者写且同时被至少一个其他使用者读或写，所以在一个值被可变引用时不允许再次被任何引用。

### 垂悬引用（Dangling References）

这是一个换了名字的概念，如果放在有指针概念的编程语言里它就指的是那种没有实际指向一个真正能访问的数据的指针（ **注意：** 不一定是空指针，还有可能是已经被释放的资源）。它们就像失去悬挂物体的绳子，所以叫“垂悬引用”。  
“垂悬引用”在Rust语言里不允许出现，如果有，编译器会发现它。  
下面是一个垂悬的典型案例：

```rust
fn main(){
    let reference_to_nothing = dangle();
}

fn dangle() -> &String{
    let s = String::from("hello");

    &s
}
```

很显然，伴随着dangle函数的结束，其局部变量的值本身没有被当作返回值，被释放了。但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现。

## Rust Slice（切片）类型

切片（Slice）是对数据值的部分引用。  
切片这个名字往往出现在生物课上，我们做样本玻片的时候要从生物体上获取切片，以供在显微镜上观察。在Rust中，切片的意思大致也是这样，只不过它从数据取材引用。

### 字符串切片

最简单、最常用的数据切片类型是字符串切片（String Slice）。

```rust
fn main(){
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{}={}+{}", s, part1, part2);
}
```

运行结果：
> broadcast=broad+cast

![字符串切片](README_files/7.jpg)

上图解释了字符串切片的原理（ **注：** Rust中的字符串类型实质上记录了字符在内存中的起始位置和其长度，暂时了解到这一点）。  
使用`..`表示范围的语法在循环章节中出现过。`x..y`表示`[x,y)`的数学含义。`..`两边可以没有运算数：

> ..y 等价于 0..y  
x.. 等价于位置x到数据结束  
.. 等价于位置0到数据结束

**注意：** 到目前为止，尽量不要在字符串中使用非英文字符，因为编码的问题。具体原因会在“字符串”章节叙述。  
被切片引用的字符串禁止更改其值：

```rust
fn main(){
    let mut s = String::from("runzm");
    let slice = &s[0..3];
    s.push_str("yes!"); // 错误
    println!("slice = {}", slice);
}
```

这段程序不正确。  
s被部分引用，禁止更改其值。  
为什么每一次都要写`String::from("runoob")`，直接写`"runoob`不行吗？  
在Rust中有两种常用的字符串类型：str和String。str是Rust核心语言类型，就是现在讲的字符串切片（String Slice），常常以引用的形式出现（&str）。  
凡是用双引号包括的字符串常量整体的类型性质都是`&str`：

```rust
let s = "hello";
```

这里的`s`就是一个`&str`类型的变量。  
String类型是Rust标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。String和str除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。  
`String`和`str`都支持切片，切片的结果是`&str`类型的数据。  

**注意：** 切片结果必须是引用类型，但开发者必须自己明示这一点：

```rust
let slice = &s[0..3];
```

有一个快速的方法可以将`String`转换成`&str`：

```rust
let s1 = String::from("hello");
let s2 = &s1[..];
```

### 非字符串切片

除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：

```rust
fn main(){
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter(){
        println!("{}", i);
    }
}
```

运行结果：
> 1  
3  
5

## Rust结构体

Rust中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据捆绑在一起形成整体，但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做“字段”。

### 结构体定义

这是一个结构体定义：

```rust
struct Site{
    domain: String,
    name: String,
    nation: String,
    found: u32
}
```

**注意：** 这里跟C/C++不一样，在Rust里struct语句仅用来定义，不能声明实例，结尾不需要`;`符号，而且每个字段定义之后用`,`分隔。

### 结构体实例

Rust很多地方受JavaScript影响，在实例化结构体的时候用JSON对象的`key: value`语法来实现定义：

```rust
let runzm = Site {
    domain: String::from("wwww.runzm.com"),
    name: String::from("RUNZM"),
    nation: String::from("China"),
    found: 2013
};
```

`struct`格式为：

```rust
    结构体类名 {
        字段名：字段值,
        ···
    }
```

这样的好处是不仅使程序更加直观，还不需要按照定义的顺序来输入成员的值。  
如果正在实例化的结构体有字段名称和现存变量名称一样的，可以这样简化书写：

```rust
let domain = String::from("www.runzm.com");
let name = String::from("RUNZM");
let runzm = Site {
    domain,  // 等同于domain: domain,
    name,   // 等同于name: name,
    nation: String::from("China"),
    found: 2013
};
```

有这样一种情况：你想要新建一个结构体的实例，其中大部分属性需要被设置成与现存的一个结构体一样，仅需要改其中的一两个字段的值，可以使用结构体更新语法：

```rust
let site = Site {
    domain: String::from("www.runzm.com"),
    name: String::from("RUNOOB"),
    ..runzm
};
```

**注意：** `..runzm`后面不可以有逗号。这种语法不允许一成不变的复制另一个结构体实例，意思就是至少重新设定一个字段的值才能引用其他实例的值。

### 元组结构体

有一种更简单的定义和使用结构体的方式: **元组结构体**。  
元组结构体是一种形式是元组的结构体。  
与元组的区别是它有名字和固定的类型格式。它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：

```rust
struct Color(u8, u8, u8);
struct Point(f64, f64);

let black = Color(0, 0, 0);
let origin = Point(0.0, 0.0);
```

“颜色”和“点坐标”是常用的两种数据类型，但如果实例化时写个大括号再加上两个名字就为了可读性牺牲了便捷性，Rust不会遗留这个问题。元组结构体对象的使用方式和元组一样，通过`.`和下标来进行访问：

```rust
fn main(){
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}
```

运行结果：
> black = (0, 0, 0)  
origin = (0, 0)

### 结构体所有权

结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。  
这就是为什么案例中使用了`String`类型而不使用`&str`的原因。  
但这不意味着结构体中不能定义引用型字段，这需要通过“生命周期”机制来实现。  
但现在还难以说明“生命周期”概念，在后面说明。  

#### 输出结构体

调试中，完整地显示出一个结构体实例是非常有用的。但如果我们手动的书写一个格式会非常的不方便。所以Rust提供了一个方便地输出一整个结构体的方法：

```rust
#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

fn main(){
    let rect1 = Rectangle{width: 30, height: 50};

    println!("rect1 is {:?}", rect1);
}
```

如第一行所示：一定要导入调式库`#[derive(Debug)]`，之后在`println`和`print`宏中就可以用`{:?}`占位符输出一整个结构体：
> rect1 is Rectangle { width: 30, height: 50 }

如果属性较多的话可以使用另一个占位符`{:#?}`。  
输出结果：
> rect1 is Rectangle {
    width: 30,
    height: 50,
}

#### 结构体方法

方法（Mothod）和函数（Function）类似，只不过它是用来操作结构体实例的。  
如果学习过一些面向对象的语言，那么一定很清楚函数一般放在类定义里并在函数中用this表示所操作的实例。  
Rust语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在Rust实现。  
结构体方法的第一个参数必须是`&self`，不需要声明类型，因为self不是一种风格而是关键字。  
计算一个矩形的面积：

```rust
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main(){
    let rect1 = Rectangle{width: 30, height: 50};

    println!("rect1's erea is {}", rect1.area());
}
```

输出结果：
> rect1's erea is 1500  

**注意：** 在调用结构体方法的时候不需要填写self，这是出于对使用方便性的考虑。

一个多参数的例子：

```rust
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

fn main(){
    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 40, height: 20};

    println!("{}", rect1.wider(&rect2));
}
```

运行结果：
> false

这个程序计算rec1是否比rec2更宽。

### 结构体关联函数

之所以“结构体方法”不叫“结构体函数”是因为“函数”这个名字留给了这种函数：它在impl块中却没有`&self`参数。  
这种函数不依赖实例，但是使用它需要声明是在哪个impl块中的。  
一直使用的`String::from`函数就是一个“关联函数”。

```rust
#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle{width, height}
    }
}

fn main(){
    let rect = Rectangle::create(30, 50);

    println!("{:#?}", rect);
}
```

运行结果：
> Rectangle {  
    width: 30,  
    height: 50,  
}

**Tip：** 结构体`impl`块可以写几次，效果相当于它们内容的拼接！

#### 单元结构体

结构体只可以作为一种象征而无需任何成员：

```rust
struct UnitStruct;
```

我们称这种没有身体的结构体为单元结构体（Unit Struct）。

## Ruct枚举类

枚举类在Rust中并不像其他编程语言中的概念那么简单，但依然可以十分简单的使用：

```rust
#[derive(Debug)]

enum Book {
    Papery, Electronic
}

fn main() {
    let book = Book::Papery;

    println!("{:?}", book);
}
```

运行结果：
> Papery

书分为纸质书（Papery book）和电子书（Electronic book）。  
如果现在正在开发一个图书管理系统，需要描述两种书的不同属性（纸质书有索书号，电子书只有URL），你可以为枚举类成员添加元组属性描述：  

```rust
enum Book {
    Papery(u32), 
    Electronic(String),
}

let book = Book::Papery(1001);
let ebook =  Book::Electronic(String::from("url://..."));
```

如果你想为属性命名，可以用结构体语法：

```rust
enum Book {
    Papery{ index: u32 },
    Electronic{ url: String},
}

let book = Book::Papery{index: 1001};
```

虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。访问的方法在match语法中。

### match语法  

枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的switch）。switch语法很经典，但在Rust中并不支持，很多语言放弃switch的原因都是因为switch容易存在因忘记添加break而产生的串接运行问题，Java和C#这类语言通过安全检查杜绝这种情况出现。  
Rust通过match语句来实现分支结构。下面是如何用match处理枚举类：

```rust
enum Book {
    Papery {index: u32},
    Electronic {url: String},
}

let book = Book::Papery{index: 1001};

match book {
    Book::Papery{ index } => {
        println!("Papery book {}", index);
    },
    Book::Electronic{ url } => {
        println!("E-book {}", url);
    }
}
```

match除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（`&str`）类型的数据进行分支选择。其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。  
对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事`.`例外情况用下划线`_`表示：

```rust
fn main() {
    let t ="abc";

    match t {
        "abc" => println!("Yes"),
        _ => {},
    }
}
```

### Option枚举类

`Option`是Rust标准库中的枚举类，这个类用于填补Rust不支持`null`引用的空白。  
许多语言支持`null`的存在（C/C++、Java），这样很方便，但也制造了极大的问题，`null`的发明者也承认了这一点，“一个方便的想法造成累计10亿美元的损失”。  
`null`经常在开发者把一切都当做不是null的时候给予程序致命一击：毕竟只要出现一个这样的错误，程序的运行就要彻底终止。  
为了解决这个问题，很多语言默认不允许`null`，但在语言层面支持`null`的出现（常在类型前面用？符号修饰）。  
Java默认支持`null`，但可以通过`@NotNull`注解限制出现`null`，这是一种应付的方法。  
Rust在语言层面彻底不允空值`null`的存在，但无奈`null`可以高效的解决少量的问题，所以Rust引入了`Option`枚举类：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

如果你想定义一个可以为空值的变量，你可以这样：

```rust
let opt = Option::Some("Hello");
```

如果你想针对`opt`执行某些操作，你必须先判断它是否是`Option::None`：

```rust
fn main(){
    let opt = Option::Some("hello");

    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}
```

运行结果：
> hello

如果变量刚开始是空值，请体谅一下编译器，它怎么知道值不为空的时候变量是什么类型呢？  
所以初始值为空的`Option`必须明确类型：

```rust
fn main(){
    let opt: Option<&str> = Option::None;

    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}
```

运行结果：
> opt is nothing

这种设计会让空值编程变得不容易，但这正是构建一个稳定高效的系统所需要的。由于`Option`是Rust编译器默认引入的，在使用时可以省略`Option::`直接写`None`或者`Some()`。  
`Option`是一种特殊的枚举类，它可以含值分支选择：

```rust
fn main(){
    let t = Some(64);

    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }
}
```

### if let 语法

```rust
fn main(){
    let i = 0;
    
    match i {
        0 => println!("zero"),
        _ => {},
    }
}
```

放入主函数运行结果：
> zero

这段程序的目的是判断`i`是否是数字`0`，如果是就打印`zero`。  
现在用`if let`语法缩短这段代码：

```rust
fn main(){
    let i = 0;
    
    if let 0 = i {
        println!("zero");
    }
}
```

`if let`语法格式如下：

```rust
if let 匹配值 = 源变量 {
    语句块
}
```

可以在之后添加一个else块来处理例外情况。  
if let语法可以认为是只区分两种情况的match语句的“语法糖”（语法糖指的是某种语法的原理相同的便捷替代品）。  
对于枚举类依然适用：  

```rust
fn main() {
    enum Book {
        Papery(u32),
        Electronic(String)
    }

    let book  = Book::Electronic(String::from("url"));

    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
```
