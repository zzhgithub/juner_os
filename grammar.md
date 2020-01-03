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
