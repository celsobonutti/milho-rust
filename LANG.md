# 🌽

Milho (corn in portuguese) is a toy dialect of Lisp written as a way to learn more about compilers.
There are implementations in [rust](https://github.com/celsobonutti/milho) and [go](https://github.com/danfragoso/milho)

## Primitives
* ### Number
```clojure
5 ;; Number
1/5 ;; Numbers can also be fractional
5/1 ;; 5/1 is the same as 5. Actually, 5 is implemented as 5/1 underneath 
```
* Boolean
```clojure
True ;; true
False ;; false
```
* String
```clojure
"Strings are written with double quotes."
```
* Error
```clojure
(make-error 404 "Not found");; Erros cannot be created out of nowhere. You can, though, create your own errors using the make-error builtin
(make-error "oops, I broke" 20);; Notice that the first argument needs to be a number, and the second, a string. Otherwise you'll get an error, but not the one you're expecting. :p
```
* List
```clojure
[ "hello" "my" "kind" "stranger" ", don't you have something better to do?" ] ;; Lists are declared with brackets and spaces between elements (yup, no commas)
[ 5 "years" ] ;; They can hold multiple data types
(: 25 [ "years" ]) ;; And you can prepend to their head like this
(++ [ "years" ] [ 25 ]) ;; And concatenated them like this
```
* Nil
```clojure
Nil ;;
```

## Defining your things :P
* ### Variables
```clojure
(def a 5) ;; Variables are declared with the def built-in
(def things-i-like [ "memes" "basimga" "xd" ]) ;; And they can hold pretty much everything ;)
```
* ### Functions
```clojure
(defn      ;; functions are declared with the defn built-in
  sum      ;; its first argument is the name of your function
  [ a b ]  ;; the second one is a list with the name of your parameters  
  (+ a b)  ;; and the third is your function per se
) 
```

## Dealing with errors
```clojure
(def res (make-error 404 "Not found"))

(if (is-error res) ;; Errors can be checked with the is-error builtin
  (print "Oops, I broke")
  (print "Oh well, I'm working")
)

(get-error-message res) ;; You can access your error's message with the get-error-message bultin
(get-error-code res) ;; Or access the code with get-error-code 
```
