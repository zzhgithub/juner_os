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


## Lisp 的语法:
- todo 

### BNF
> 在双引号中的字(“word”)代表着这些字符本身。而double_quote用来代表双引号。
> 在双引号外的字（有可能有下划线）代表着语法部分。
> 尖括号( < > )内包含的为必选项。
> 方括号( [ ] )内包含的为可选项。
> 大括号( { } )内包含的为可重复0至无数次的项。
> 竖线( | )表示在其左右两边任选一项，相当于”OR”的意思。
> ::= 是“被定义为”的意思。

基本s表达式生成语法

```bnf
s_expression ::= atomic_symbol |
               | "(" s_expression "."s_expression ")" |
               | list 
   
list ::= "(" s_expression < s_expression > ")"

atomic_symbol ::= letter atom_part

atom_part ::= empty | letter atom_part | number atom_part

letter ::= "a" | "b" | " ..." | "z"

number ::= "1" | "2" | " ..." | "9"

empty ::= " "
```

TODO!!问题上面的定义不能很好的识别出字符串作为元素!!!


# 参考资料
- (Lisp Bnf)[https://iamwilhelm.github.io/bnf-examples/lisp]
- (Json 识别状态机 【参考用】)[https://www.json.org/json-en.html]