# `03_ownership_borrowing`

## 学习目标

通过八个小练习，按从移动到切片的顺序理解移动、`Copy`、共享借用、可变借用、`clone`、返回拥有所有权的值和字符串切片。

## 练习顺序

1. `01_copy_and_move`：比较整数的 `Copy` 和 `String` 的移动。
2. `02_borrow_string`：通过 `&str` 共享借用文本，让调用者继续使用原来的 `String`。
3. `03_mutable_string`：通过 `&mut String` 在独占、临时访问期间追加后缀。
4. `04_push_vec`：通过 `&mut Vec<String>` 添加项目，并为向量创建需要的拥有所有权的 `String`。
5. `05_retain_numbers`：可变借用 `Vec<i32>`，原地保留非负整数；本练习会就地介绍 `retain` 所用的闭包语法 `|number| condition`。
6. `06_clone_when_needed`：两个位置都需要拥有同一字符串时，明确且有意地使用 `clone`。
7. `07_return_string`：从 `&str` 构造并返回拥有所有权的 `String`。
8. `08_first_word`：借用字符串切片并返回第一个单词。

## 前置条件

开始前请完成 `02_functions_collections`，并准备阅读编译器关于移动和借用的错误信息。

## 下一章

完成本章后进入 `04_errors`。
