# solutions/

这里是与 `exercises/` 路径对应的 27 个可运行参考实现，所有实现都只使用 Rust 标准库。

请把 `solutions/` 当作对照材料，而不是第一次尝试的入口：先阅读 starter 的测试和注释，自己运行并修复练习，再用参考实现比较思路。

检查全部参考实现的命令是：

```sh
sh check.sh solutions --run-all
```

也可以使用快捷脚本；它会按脚本位置找到仓库，因此不要求当前目录是仓库根目录：

```sh
sh scripts/verify_solutions.sh
```

也可以把某一个章节或练习目录传给 `check.sh`，例如：

```sh
sh check.sh solutions/00_hello
```

`cargo check --workspace` 只检查 54 个 workspace package 能否编译，不能替代上述行为测试。exercise starter 在新 checkout 中故意不完整；这里的 solution 检查才是应当全部通过的参考实现验证。
