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

## 学习方式

先进入当前章节的 exercise README，按编号完成练习。只运行一个练习时使用这个命令：

```sh
cargo test --manifest-path exercises/03_ownership_borrowing/03_first_word/Cargo.toml
```

完成 starter 后，可以检查全部参考实现：

```sh
sh check.sh solutions --run-all
```

所有练习都只使用 Rust 标准库。`check.sh` 会按章节和编号运行测试；后续任务会补充更多包和检查目标。

## 所有权章节的八项重点

`03_ownership_borrowing` 用八个练习集中练习：借用 `String`、返回所有权、字符串切片、可变借用 `String`、原地修改 `Vec<i32>`、区分 `Copy` 与移动、只在确实需要时使用 `clone`，以及区分 UTF-8 字节长度和 Unicode 字符数量。

每次遇到编译器错误，先阅读错误指出的移动、借用或生命周期关系，再决定是否需要改变代码；不要把 `clone` 当作默认修复方式。
