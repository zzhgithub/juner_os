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


### prn

打印一个符号如果不存在就报错

```lisp
(prn abc)
=> not found 'abc'
```

### quote
表示后面的值是这个符号本身
可以和prn进行配合使用

```lisp
(prn (quote abc))

=> abc
=> Nil
```
解释：打印abc是prn函数的副作用。prn函数真正的返回是Nil。

### '

' 是 quote的语法糖。
'abc 和 (quote abc)是完全等效的。实际上它在解释器内部也会翻译成quote的形式。


### quasiquote 、 unquote 和 splice-unquote
quasiquote建立一个可以临时求值的的符号。如果单独使用和quote没有什么区别。  
要和unquote与splice-unquote联合使用。其中有轻微的差别。unquote表示对下一个符号进行临时取值。  
splice-unquote临时取值后把列表展开。

具体例子如下：
```lisp
(def! lst '(2 3))
=> (2 3)

(quasiquote (1 (unquote lst)))
=> (1 (2 3))

(quasiquote (1 (splice-unquote lst)))
=> (1 2 3)
```

### ` 、 ～ 和 ～@
quasiquote 、 unquote 和 splice-unquote 的语法糖。

```lisp
(def! lst '(2 3))
=> (2 3)

`(1 ～lst)
=> (1 (2 3))

`(1 ~@lst)
=> (1 2 3)
```

### cons
这个函数将它的第一个参数连接到它的第二个参数 (一个列表) 前面，返回一个新列表。

```lisp
(cons [1] [2 3])
=> ([1] 2 3)

(cons 1 [2 3])
=> (1 2 3)
```

### concat
这个函数接受零个或多个列表作为参数，并且返回由这些列表的所有参数组成的一个新列表。

```lisp
(concat [1 2] (list 3 4) [5 6])
=> (1 2 3 4 5 6)

(concat [1 2])
=> (1 2)
```

### defmacro! 和 macroexpand
宏定义和宏展开

宏定义 定义一个符号。它的返回值会被继续当做ast进行求值。所有这里可以广泛的运用到之前的' ` ～ ～@ d等语法糖。  
宏展开。展开一个宏，只计算出它要求值的ast而不进行求值。
```lisp
(defmacro! unless (lamdba (pred a b) `(if ~pred ~b ~a)))

=> ...（此处省略）

(unless false 7 8)
=> 7

(macroexpand (unless false 7 8))
=> (if fasle 7 8)
```

### nth
这个函数接受一个列表（或向量）以及一个数字（序号）作为参数，返回列表中给定序号位置的元素。如果序号超出了返回，函数抛出一个异常。

```lisp
(nth [1 2 3] 0)
=> 1

(nth '(1 2 3) 1)
=> 2
```

### first
这个函数接受一个列表（或向量）作为参数，返回它的第一个元素，如果列表（或向量）是空的，或者参数本身是 nil，则返回 nil。

```lisp
(first '((1 2) 2 3))

=> (1 2)
```

### count
接受一个列表或者向量返回列表或者向量的长度

```lisp
(count '(1 2 (2 3)))
=> 3

(count [1 2 3])
=> 3
```

### empty?
接受一个列表或者向量判断这个对象是否事空。如果我空的情况下返回true否则返回false

```lisp
(empty? '())
=> true

(empty? nil)
=> true
```
