// 所有权规则：
// Rust中的每个值都有一个所有者（变量）
// 值在任何时刻有且只有一个所有者
// 当所有者离开作用域，这个值将被丢弃

fn main() {
    {
        // s 无效，尚未声明

        let s = "hello"; // 从此处起，s是有效的

        // 使用 s
        println!("{s}");
    } // 此作用域结束，s 不再有效

    {
        // s 无效，尚未声明

        let mut s = String::from("hello, "); // 从此处起，s是有效的
        // 使用 s
        s.push_str("world!");
        println!("{s}");
    } // 此作用域结束，s 不再有效

    {
        let x = 5;
        let y = x;
        println!("[copy] x = {}, y = {}", x, y);
        // 整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
    }

    {
        // 变量与数据交互的方式（二）：移动
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("s1 = {}, s2 = {}", s1, s2);
        println!("[move] s2 = {}", s2);

        /*
        error[E0382]: borrow of moved value: `s1`
                --> src/main.rs:35:38
                |
                33 |         let s1 = String::from("hello");
                |             -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
                34 |         let s2 = s1;
                |                  -- value moved here
                35 |         println!("s1 = {}, s2 = {}", s1, s2);
                |                                      ^^ value borrowed here after move
                |
                = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        */

        /*
        当我们将 s1 赋值给 s2，String 的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据。
        */
    }

    {
        // 变量与数据交互的方式（二）：克隆
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("[clone] s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s = String::from("hello"); // s 进入作用域
        takes_ownership(s); // s 的值移动到函数里 ...
        // ... 所以到这里不再有效
    }

    {
        let s1 = String::from("hello");
        let s2 = takes_and_gives_back(s1);
        // s1 被移动到
        // takes_and_gives_back 中,
        // 它也将返回值移给 s2
        println!("[takes_and_gives_back] s2 = {}", s2);
    }   // 这里, s2 移出作用域并被丢弃
}

fn takes_ownership(some_string: String) {
    println!("[takes_ownership] some_string = {}", some_string);
}   // 这里，some_string 移出作用域并调用 `drop` 方法。
// 占用的内存被释放

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
