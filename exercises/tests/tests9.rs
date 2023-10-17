// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

// tests9.rs
//
// Rust非常擅长与C/C++和其他静态编译语言共享FFI接口，甚至可以在代码内部进行链接！它通过`extern`块来实现，就像下面的代码一样。
//
// `extern`关键字后面的短字符串指示外部导入函数将遵循的ABI。在这个练习中，使用"Rust"，而其他变种如标准C ABI的"C"，Windows ABI的"stdcall"也存在。
//
// 外部导入函数在`extern`块中声明，用分号标记签名的结尾，而不是用花括号。可以对这些函数声明应用一些属性，以修改链接行为，比如使用#[link_name = ".."]来修改实际的符号名称。
//
// 如果要将符号导出到链接环境，还可以在函数定义前使用相同的ABI字符串注释标记`extern`关键字。Rust函数的默认ABI实际上是"Rust"，因此如果要链接到纯Rust函数，可以省略整个`extern`术语。
//
// Rust默认情况下对符号进行名称重整（mangling），就像C++一样。为了抑制此行为并使这些函数能够通过名称寻址，可以应用属性#[no_mangle]。
//
// 在这个练习中，你的任务是使测试用例能够调用模块Foo中的`my_demo_function`。`my_demo_function_alias`是`my_demo_function`的别名，因此测试用例中的两行代码应该调用相同的函数。
//
// 除了添加两行属性之外，不应修改任何现有代码。


extern "Rust"  {
    fn my_demo_function(a: u32) -> u32;

    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
     fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[no_mangle]
    #[export_name = "name"]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        // 外部导入的函数默认情况下是不安全的
        // 因为它们来自不受信任的其他语言的源。你可以
        // 将它们封装在安全的Rust API中，以减轻调用者的负担。
        //
        // 安全性：我们知道这些函数是安全的Rust函数的别名。

        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
