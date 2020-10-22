# juner_os

![juner_os](uefi.png)

[English](.README.md)

# 简介
这个项目结合了[blog_os](https://os.phil-opp.com/)和[mal](https://github.com/kanaka/mal)两个项目的内容。  
现在实现了一个可以运行交互的lisp shell.后续目标是使用lisp和rust混合的方式组织操作系统的代码。并而核心库准备使用lisp进行加载和维护。项目进行中。


# 特性
- rust 实现的内核
- LISP REPL

# 依赖
- rustc 1.48.0-nightly 
- qemu


# 运行
```
make all
```

# Lisp 的语法:
- [语法](./grammar_zh.md)


# 梦想清单
- [ ] VGA text mode 下显示打印
  - [x] 光标跟随
  - [x] 删除
  - [ ] 代码提示Tab
  - [ ] 光标移动编辑
  - [ ] 滚动条
- [ ] Lisp 完整功能
  - [x] 支持宏
  - [ ] 支持代码提示
  - [ ] sacnf方法
  - [ ] 文件读写
  - [ ] 模块加载
  - [ ] JIT
  - [ ] 支持call/cc
- [ ] 设备
  - [ ] 识别硬盘
  - [ ] 声音驱动
  - [ ] 网卡支持
- [ ] 抽象的功能
  - [x] UEFI 支持
  - [ ] 并行多任务
  - [ ] 支持GUI
  - [ ] 网络加载lisp模块
  - [ ] 多核心利用
  - [ ] 自举（支持rust环境）
- [ ] 梦中的app
  - [ ] MAL 这个方言的编辑器
  - [ ] NoteBook

# 现在的工作和目标

目前来说实现的功能还很初步，lisp内有没有实现的机制，内核里有要实现的内容。现在在研究使用UEFI进行引导，让系统可以读写文件，并在lisp内运行。
后面uefi的分支会合并到主分支上，并且作为主流。

# 期待你加入并完善这个项目

联系我: zzhggmm@gmail.com