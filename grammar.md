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

### try* catch* 和 throw

try-catch代码块和其他 语言的try-catch并没有什么特别的不同。try后面跟着一个语句。如果有多条语句可以使用do语句进行配合，而catch
语句中，会有一个异常，并且可以控制代码段的返回。如果使用了throw可以主动的抛出一个异常。

```lisp
(throw "err1")
=> err1

(try* abc (catch* exc (prn "exc is:" exc)))
=> "exc is:" "not found 'abc'"

(try* (throw "my exception") (catch* exc (do (prn "exc:" exc) 7)))
=> "exc:" "my exception"
=> 7
```

### apply
它有两个参数，第一个参数是一个函数。而第二个参数是这个函数需要的入参的列表。这个函数会返回这个求值的结果。

```lisp
(apply + (list 1 3))
=> 4

(apply + '(2 3))
=> 5

(apply (lamdba [x y] (do (prn  x "+" y) (+ x y))) '(7 8))
=> 7 "+" 8
=> 15
```

### map
它有两个参数，第一个参数是一个函数，第二个参数是一个向量或者列表。之后它会对第二个参数中的每个值进行求值，然后结果返回一个新的列表。  
注意map每个函数只接受一个参数。但是可以配合apply达到传多个值的效果。

```lisp

(map (lamdba [x] (apply + x)) (list [1 2] [2 3]))
=> (3 5)

```


### 几个简单的类型判断函数
- nil?
- true?
- false?
- symbol? (判断是不是符号)


### Atom 进行原子操作

- atom: 输入一个 mal 值，并返回一个新的指向这个值的原子。
- atom?: 判断输入的参数是不是原子，如果是，返回 true。
- deref: 输入一个原子作为参数，返回这个原子所引用的值。
- reset!: 输入一个原子以及一个 mal 值，修改原子，让它指向这个 mal 值，并返回这个 mal 值。
- swap!: 输入一个原子，一个函数，以及零个或多个函数参数。将原子的值作为第一参数，并将余下的函数参数作为可选的参数传输函数中，将原子的值置为函数的求值结果。返回新的原子的值。(边注: Mal是单线程的，但在像Clojure之类的并发语言中，swap!将是一个原子操作，(swap! myatom (fn* [x] (+ 1 x)))总是会把myatom计数增加1，并且在原子被多个线程操作时不会导致结果出错)

```lisp
(def! *test-atom* (atom 0))
=> (atom 0)

(reset! *test-atom* 10)
=> 10

(deref *test-atom*)   | @*test-atom*
=> 10

(swap! *test-atom* (lamdba [x] (+ x 1)))
=> 11
```

@ 是deref的语法糖


### Hash-Map 的操作和 关键字
在此方言中hashmap是一种基本的类型。使用{}大括号表示字面量。其中key可以使用两种方式。字符串和关键字。关键字是使用：开头的字符串。  
{"a" 1 "b" 2}
{:a 1 :b 2}
上面两种形式都可以表示一个map.  
- hash-map: 接受偶数数量的参数，返回一个新的 mal 哈希表，其中键为奇数位置的参数，它们的值分别为与之对应的偶数位置的参数，它基本上是 {}reader 字面语法的函数形式。
- map?: 接受一个参数，如果参数是哈希表的话，返回 true(mal 中的 true 值)，否则返回 false(mal 中的 false 值)
- assoc: 接受一个哈希表作为第一个参数，余下的参数为需要关联(合并)到哈希表里的奇/偶-键/值对。注意，原始的哈希表不会被修改(记住，mal 的值是不可变的)，旧哈希表中的键/值与参数中的键/值对合并而成的新的哈希表作为结果返回。
- dissoc:接受一个哈希表作为第一个参数，余下的参数为需要从哈希表中删除的键。与前面一样，注意原始的哈希表是不变的，只是把删除了参数中指定的键的新哈希表返回出来。参数列表中在原哈希表不存在的键会被忽略。
- get: 接受一个哈希表和一个键，返回哈希表中与这个键对应的值，如果哈希表中不存在这个键，则返回 nil。
- contains?: 接受一个哈希表和一个键，如果哈希表中包含这个键，则返回 true(mal 中的 true 值)，否则返回 false(mal 中的 false 值)。
- keys: 接受一个哈希表，并返回一个列表(mal 中的 列表值)，其中包含了哈希表中的所有的键。
- vals: 接受一个哈希表，并返回一个列表(mal 中的 列表值)，其中包含了哈希表中的所有的值。 

注意上面的方法都没有产生副作用，也就是没有改变原理的值。如果你要这么做的话建议你使用atom或者重新def!



### gensym 生成一个系统中全新的符号
```lisp
(gensym)
=> 每次运行不一样！
```
