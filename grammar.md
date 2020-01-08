# Lisp 语法

## 数据类型
- 布尔型 true/false
- 空类型 nil


## 给符号绑定“值”（S表达式）

### def!
```lisp
（def! x1 S)
```

- x1 是要绑定的符号 
- S 要绑定的S表达式

example: 
```lisp

(def! mynum 111)
=> 111
;; 此时绑定值到了mynum 这个符号上
```

### let*

```lisp
(let* (p (+ 2 3) 
       q (+ 2 p)) (+ p q))
=> 12
```
使用let* 临时赋值


### lamdba

```
((lamdba [x] (+ 1 x)) 1)
=> 2
```
定义一个闭包过程。可以和def!组合起来定义一个函数。

### do

```lisp
(do (+ 1 2) (+ 3 4) 5)
=> 5
```
计算列表中的每个元素的值，然后返回最后一个元素的值。

### list

```lisp
(list 1 2 3)
=> (1 2 3)
```

生成一个Lisp的列表。

### read-string

```lisp
(read-string "Nil")
=> Nil

(read-string "(+ 1 1)")
=> (Fn*<xxx00> 1 1)

```
读一个字符串生成一个Lisp对象，注意只生成对象，但是不进行求值。

### eval

```lisp
(eval (read-string "(+ 1 3)"))
=> 4

(eval (list + 1 3))
=> 4

```
对Lisp对象进行求值。使用这个方法后，Lisp中没有了数据和代码的界限。捅破了数据和代码的那层窗户纸。