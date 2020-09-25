# Lisp Syntax

## Data type
- Boolean true/false
- Empty type nil

## Bind "values" to symbols (S-expressions)

### def!
```lisp
（def! x1 S)
```

- x1 is the symbol to bind to.
- S The S expression to bind to.

example:
```lisp

(def! mynum 111)
=> 111
;; This binds the value to the mynum symbol.
```

### let*

```lisp
(let* (p (+ 2 3) 
       q (+ 2 p)) (+ p q))
=> 12
```
Temporary assignment using let*

### lambda

```
((lambda [x] (+ 1 x)) 1)
=> 2
```
Define a closure procedure. It can be combined with def! to define a function.

Note the syntax of arguments If you use the & symbol before an argument, you can indicate that multiple arguments are in a list. It can only be used before the last argument.

### do

```lisp
(do (+ 1 2) (+ 3 4) 5)
=> 5
```
Calculates the value of each element in the list and then returns the value of the last element.

### list

```lisp
(list 1 2 3)
=> (1 2 3)
```

Generate a list of Lisp.

### read-string

```lisp
(read-string "Nil")
=> Nil

(read-string "(+ 1 1)")
=> (Fn*<xxx00> 1 1)

```

Read a string to generate a Lisp object, note that only the object is generated, but no evaluation is done.

### eval

```lisp
(eval (read-string "(+ 1 3)"))
=> 4

(eval (list + 1 3))
=> 4

```

Evaluate the Lisp object. With this method, there is no boundary between data and code in Lisp. The layer of data and code is poked through.


### prn

Print a symbol and report an error if it does not exist

```lisp
(prn abc)
=> not found 'abc'
```

### quote
indicates that the value that follows is the symbol itself.
Can be used in conjunction with prn

```lisp
(prn (quote abc))

=> abc
=> Nil
```
Explanation: printing abc is a side-effect of the prn function. the real return of the prn function is Nil.

### '

' is the grammatical sugar of quote.
'abc and (quote abc) are completely equivalent. It actually translates into quote form inside the interpreter as well.

### quasiquote, unquote and splice-unquote
quasiquote creates a notation that can be evaluated temporarily. If used alone there is no difference between unquote and quote.  
To be used in conjunction with unquote and splice-unquote.There is a minor difference. unquote means that the next symbol is temporarily evaluated.  
A splice-unquote takes a temporary value and then expands the list.


Specific examples are as follows.
```lisp
(def! lst '(2 3))
=> (2 3)

(quasiquote (1 (unquote lst)))
=> (1 (2 3))

(quasiquote (1 (splice-unquote lst)))
=> (1 2 3)
```

### ` 、 ～ 和 ～@
grammatical sugar of quasiquote 、 unquote and splice-unquote ;

```lisp
(def! lst '(2 3))
=> (2 3)

`(1 ～lst)
=> (1 (2 3))

`(1 ~@lst)
=> (1 2 3)
```

### cons
This function connects its first argument to its second argument (a list) and returns a new list.

```lisp
(cons [1] [2 3])
=> ([1] 2 3)

(cons 1 [2 3])
=> (1 2 3)
```

### concat
This function accepts zero or more lists as arguments and returns a new list consisting of all the arguments of those lists.

```lisp
(concat [1 2] (list 3 4) [5 6])
=> (1 2 3 4 5 6)

(concat [1 2])
=> (1 2)
```

### defmacro! 和 macroexpand
Macro definition and macro expansion

Macro Definition Defines a symbol. Its return value will continue to be evaluated as AST. All of which can be used extensively with the previous syntactic sugars such as ' ` ～ ～ @ d etc.  
Macro expansion. Expands a macro to calculate only the last of its required values without evaluating it.

```lisp
(defmacro! unless (lambda (pred a b) `(if ~pred ~b ~a)))

=> ...(Omitted here)

(unless false 7 8)
=> 7

(macroexpand (unless false 7 8))
=> (if fasle 7 8)
```

### nth
This function takes a list (or vector) and a number (ordinal number) as arguments and returns the element of the list at the given ordinal position. If the ordinal number is exceeded, the function throws an exception.

```lisp
(nth [1 2 3] 0)
=> 1

(nth '(1 2 3) 1)
=> 2
```

### first
This function accepts a list (or vector) as an argument and returns its first element, or nil if the list (or vector) is empty, or if the argument itself is nil.

```lisp
(first '((1 2) 2 3))

=> (1 2)
```

### count
Accepts a list or vector and returns the length of the list or vector.

```lisp
(count '(1 2 (2 3)))
=> 3

(count [1 2 3])
=> 3
```

### empty?
Accept a list or vector to determine if the object is empty. Returns true if I'm empty and false otherwise.

```lisp
(empty? '())
=> true

(empty? nil)
=> true
```

### try* catch* 和 throw

The try-catch block is no different than try-catch in other languages. try is followed by a statement. If there are multiple statements that can be matched with a do statement, the catch
statement, an exception is thrown, and the return of the code segment can be controlled. If throw is used an exception can be actively thrown.

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
It has two arguments, the first argument is a function. The second argument is a list of the required parameters for the function. This function will return the result of this evaluation.


```lisp
(apply + (list 1 3))
=> 4

(apply + '(2 3))
=> 5

(apply (lambda [x y] (do (prn  x "+" y) (+ x y))) '(7 8))
=> 7 "+" 8
=> 15
```

### map
It has two arguments, the first argument is a function and the second is a vector or a list. It then evaluates each value in the second argument, and the result returns a new list.  
Note that map accepts only one argument per function. However, it is possible to pass multiple values with apply.

```lisp

(map (lambda [x] (apply + x)) (list [1 2] [2 3]))
=> (3 5)

```

### A few simple type determination functions
- nil?
- true?
- false?
- symbol? (Determine if it's a symbol.)


### Atomic manipulation by Atom

- atom: Enter a value of mal, and return a new atom pointing to it.
- atom?: Determines whether the argument is an atom or not, and returns true if it is.
- deref: Enter an atom as an argument, and return the value referenced by the atom.
- reset!: Enter an atom and a mal value, modify the atom to point to the mal value, and return the mal value.
- swap!: Enter an atom, a function, and zero or more function arguments. Take the value of the atom as the first argument, and transfer the remaining function arguments to the function as optional arguments, setting the value of the atom as the result of the function's evaluation. Return the new value of the atom. (Side note: Mal is single-threaded, but in a concurrent language like Clojure, swap! will be an atomic operation, and (swap! myatom (fn* [x] (+ 1 x)) will always increase the myatom count by 1, and will not cause an error in the result when the atom is manipulated by multiple threads)

```lisp
(def! *test-atom* (atom 0))
=> (atom 0)

(reset! *test-atom* 10)
=> 10

(deref *test-atom*)   | @*test-atom*
=> 10

(swap! *test-atom* (lambda [x] (+ x 1)))
=> 11
```

@ is grammatical sugar of deref

### Hash-Map Operations and keywords

In this dialect hashmap is a basic type. Use {} curly brackets to represent literal quantities. Where key can be used in two ways. Strings and keywords. The keyword is a string using: beginning.  
{"a" 1 "b" 2}
{:a 1 :b 2}
Both of the above forms can be used to represent a map.  
- hash-map: accepts an even number of arguments and returns a new mal hash table, where the keys are the arguments at odd positions and their values are the arguments at even positions, which is essentially a function of the {}reader literal syntax.
- map?: accepts an argument, returns true if it is a hash, or false if it is a mal.
- assoc: accepts a hash table as the first argument, and the remaining arguments are the odd/even-key/value pairs that need to be associated (merged) into the hash table. Note that the original hash table will not be modified (remember, the value of mal is immutable) and the new hash table is returned as a result of merging the keys/values in the old hash table with the key/value pairs in the arguments.
- dissoc: accepts a hash table as the first argument, with the remaining arguments being the keys that need to be removed from the hash table. As before, note that the original hash table is unchanged, except that the new hash table is returned with the key specified in the argument removed. Keys in the parameter list that did not exist in the original hash table are ignored.
- get: accepts a hash table and a key, returns the value corresponding to the key in the hash table, or nil if the key does not exist in the hash table.
- contains?: accepts a hash table and a key, returns true if the hash table contains the key, otherwise returns false if the key is false.
- keys: accepts a hash table and returns a list (the list value in mal) containing all the keys in the hash table.
- vals: Accepts a hash table and returns a list (of values in mal) containing all the values in the hash table. 

Note that none of the above methods have side effects, i.e., they do not change the values of the principle. If you're going to do this it is recommended that you use atom or redef!

### gensym generates a new symbol for the system

```lisp
(gensym)
=> It's different every run!
```

### cond  multiconditional
There are an even number of arguments one for the condition and one for the returned value.
```lisp
(def! ten-test (lambda [data]
                (cond
                    (> data 10) 1
                    (= data 10) 0
                    (< data 10) -1)))

(ten-test 15)
=> 1
```

TODO others baisc function