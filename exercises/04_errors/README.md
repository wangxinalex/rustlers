# `04_errors`

## 学习目标

用 `Option` 表示缺失，用 `Result` 表示可处理的错误，并练习解析和用 `?` 传播错误。

## 练习顺序

1. `01_find_price`：找不到商品时返回 `Option`。
2. `02_parse_quantity`：把数字解析为 `Result<u32, String>` 并提供友好回退。
3. `03_parse_pair`：用 `?` 传播一对数字的解析错误。
4. `04_parse_setting`：校验配置行并区分无效输入和成功结果。

## 前置条件

开始前请完成 `03_ownership_borrowing`，并理解引用如何避免不必要的所有权移动。

## 下一章

完成本章后进入 `05_mini_cli`。
