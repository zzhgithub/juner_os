# juner_os


## 直接测试运行

```
cargo xrun
```

##  使用rust-gdb 进行断点执行

```
./gdb.sh
./touchme.sh

```
### 断点指令
- b  打断点
- n  执行下一行( 这个方法比较哈！ 只能执行当前文件的一行)
- tui enable 打开源代码视图
- print 打印变量值
- c 继续执行到下一个断点
- s 运行到下一步（但是没有进入到内联汇编的汇编中 比较遗憾）



## todo
- 正常输入输出
- 多线程支持
- lisp REPL基础实现