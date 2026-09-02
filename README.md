# Rustlers

Rustlers 是一个标准库优先的 Rust 练习仓库。每个练习都是一个小型独立包，学习者先修复 `exercises/` 中不完整的 starter，再用检查脚本验证结果。

## 开始之前

请先确认已安装 Rust，并运行：

```sh
rustc --version
cargo --version
```

按下面的顺序完成六章。starter 是故意不完整的；`solutions/` 是参考答案，不应作为第一次尝试的入口。

| Chapter | Focus | Exercises |
|---|---|---:|
| `00_hello` | program shape, printing, variables, mutability, types | 4 |
| `01_control_flow` | expressions, conditions, loops, `match` | 4 |
| `02_functions_collections` | functions, `Vec`, `String`, iteration | 5 |
| `03_ownership_borrowing` | moves, `Copy`, `clone`, references, slices, mutable borrowing, UTF-8 strings | 8 |
| `04_errors` | `Option`, `Result`, parsing, `?` | 4 |
| `05_mini_cli` | file input and text statistics as an applied capstone | 2 |

## 27 项练习清单

| # | 路径 | 练习重点 |
|---:|---|---|
| 1 | `00_hello/01_print_line` | 用 `fn main` 和 `println!` 打印问候语；实现 `greeting()`。 |
| 2 | `00_hello/02_variables` | 用 `let` 绑定值，并实现 `welcome_message()`。 |
| 3 | `00_hello/03_mutability` | 用 `let mut` 修改价格，完成 `add_tax()`。 |
| 4 | `00_hello/04_types` | 使用显式 `u32` 类型完成 `receipt_total()`。 |
| 5 | `01_control_flow/01_if_expression` | 用 `if` 表达式分类温度，实现 `classify_temperature()`。 |
| 6 | `01_control_flow/02_for_sum` | 用 `for` 循环求和，实现 `sum_up_to()`。 |
| 7 | `01_control_flow/03_while_countdown` | 用 `while` 循环生成倒计时，实现 `countdown()`。 |
| 8 | `01_control_flow/04_match_command` | 用 `match` 处理命令和通配分支，实现 `command_label()`。 |
| 9 | `02_functions_collections/01_parameters` | 定义带参数和返回值的函数，实现 `greet()`。 |
| 10 | `02_functions_collections/02_push_vec` | 通过可变借用向 `Vec<String>` 添加项目。 |
| 11 | `02_functions_collections/03_slice_sum` | 借用切片并遍历求和，实现 `total()`。 |
| 12 | `02_functions_collections/04_word_count` | 用空白分割统计单词，实现 `word_count()`。 |
| 13 | `02_functions_collections/05_shopping_total` | 汇总价格并应用整数折扣，实现 `shopping_total()`。 |
| 14 | `03_ownership_borrowing/01_borrow_string` | 通过 `&String` 借用文本并读取长度。 |
| 15 | `03_ownership_borrowing/02_return_string` | 从 `&str` 构造并返回拥有所有权的 `String`。 |
| 16 | `03_ownership_borrowing/03_first_word` | 借用字符串切片，跳过前导空白并取第一个单词。 |
| 17 | `03_ownership_borrowing/04_mutable_string` | 通过 `&mut String` 原地追加后缀。 |
| 18 | `03_ownership_borrowing/05_retain_numbers` | 通过可变借用原地保留非负整数。 |
| 19 | `03_ownership_borrowing/06_copy_and_move` | 对比 `i32` 的 `Copy` 和 `String` 的移动。 |
| 20 | `03_ownership_borrowing/07_clone_when_needed` | 只有两个位置都需要所有权时才使用 `clone`。 |
| 21 | `03_ownership_borrowing/08_unicode_lengths` | 区分 Unicode 字符数量和 UTF-8 字节长度。 |
| 22 | `04_errors/01_find_price` | 用 `Option` 表示找得到或找不到价格。 |
| 23 | `04_errors/02_parse_quantity` | 用 `Result` 解析正整数并报告错误。 |
| 24 | `04_errors/03_parse_pair` | 分割并用 `?` 传播两个整数的解析错误。 |
| 25 | `04_errors/04_parse_setting` | 校验 `port=value` 配置并解析端口。 |
| 26 | `05_mini_cli/01_text_stats` | 统计文本的行数、单词数和 UTF-8 字节数。 |
| 27 | `05_mini_cli/02_file_report` | 读取文件并输出一行文本统计报告。 |

## 学习方式与检查命令

先进入当前章节的 exercise README，按编号完成练习。直接运行一个练习的命令模板是：

```sh
cargo test --manifest-path exercises/<chapter>/<exercise>/Cargo.toml
```

例如：

```sh
cargo test --manifest-path exercises/03_ownership_borrowing/03_first_word/Cargo.toml
```

`check.sh` 接受一个目标路径和可选的 `--run-all`：

```sh
sh check.sh                                      # 默认检查 exercises
sh check.sh exercises/03_ownership_borrowing/03_first_word
sh check.sh exercises --run-all                  # 失败后继续并汇总
sh check.sh solutions --run-all                  # 检查全部参考实现
sh scripts/verify_solutions.sh                  # 参考实现快捷入口
```

默认模式在第一个失败处停止；`--run-all` 会检查所有包并打印通过数量。直接的 `cargo test` 用于一个包的行为测试；`cargo check --workspace` 只验证全部 54 个 workspace package 能否编译，不会把不完整的 starter 变成已解题，也不会代替运行练习测试。新 checkout 中 exercise 行为测试故意不全通过；完成标记的代码后再检查对应练习。`solutions/` 中的参考实现应通过全部 27 个包的检查。

所有练习都只使用 Rust 标准库，不需要下载第三方依赖。

## 所有权章节的八项重点

`03_ownership_borrowing` 用八个练习集中练习：借用 `String`、返回所有权、字符串切片、可变借用 `String`、原地修改 `Vec<i32>`、区分 `Copy` 与移动、只在确实需要时使用 `clone`，以及区分 UTF-8 字节长度和 Unicode 字符数量。

建议在六章中为 `03_ownership_borrowing` 额外留出时间：它是后续 `Result` 和文件 CLI 练习的基础。每次遇到编译器错误，先阅读错误指出的移动、借用或生命周期关系，再决定是否需要改变代码；不要把 `clone` 当作默认修复方式。
