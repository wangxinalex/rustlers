# `03_ownership_borrowing`

## 学习目标

通过八个小练习理解移动、`Copy`、`clone`、引用、切片、可变借用，以及 UTF-8 字符串的长度规则。

## 练习顺序

1. `01_borrow_string`：通过引用传递 `String`，让调用者继续使用它。
2. `02_return_string`：从构造问候语的函数返回所有权。
3. `03_first_word`：借用字符串切片并返回第一个单词。
4. `04_mutable_string`：通过 `&mut String` 追加后缀。
5. `05_retain_numbers`：可变借用 `Vec<i32>`，原地移除负数。
6. `06_copy_and_move`：比较整数的 `Copy` 和 `String` 的移动。
7. `07_clone_when_needed`：两个调用者都需要所有字符串时再使用 `clone`。
8. `08_unicode_lengths`：分别统计 Unicode 字符和 UTF-8 字节。

## 前置条件

开始前请完成 `02_functions_collections`，并准备阅读编译器关于移动和借用的错误信息。

## 下一章

完成本章后进入 `04_errors`。
