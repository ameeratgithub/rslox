## Keywords
Keywords have special meanings in any programming language and should be treated according to rules of the language. **Rslox** currently supports following keywords. 

- and
- else
- false
- for
- fun
- if
- nil 
- or
- print
- return
- true
- var
- while

#### `and`
It's a binary operator which requires two operands. `and` operator returns first operand if it's falsey, otherwise returns second operand.

```javascript
// Will print 2
print 1 and 2;
// Will print nil
print nil and 2;
// Will print nil
print 2 and nil;
// Will print true
print true and true;
// Will print false
print true and false;
// Will print false
print false and true;
```

#### `else`
It is used after `if` block, to execute some code when condition in `if` is falsey.

```javascript
if (nil){}
// else will be executed
else {print "Nil\n";}

if (false){}
// else will be executed
else {print "False\n";}
```

#### `false`
This represents the `false`, a boolean constant. This value is returned when an expression is evaluated as false. All of the following expressions should be evaluated as false, and will print `false`.

```javascript
print nil==2;
print "\n";
print false==nil;
print "\n";
print 6>=10;
print "\n";
```

#### `for`
This keyword is used for `for` loop. It expects 1 declaration statement and 2 expressions to be evaluated in parenthesis. It will run until second expression, which is a condition, is false.

```javascript
for (var i=0;i<5;i=i+1){
    print i + " ";
}
```

#### `fun`
This keyword is used to declare functions. It expects function name, and parameters in parenthesis. Function must also have a body surrounded by curly braces.
```javascript
fun add(a,b){
    print a + b;
}
```

#### `if`
This keyword defines a statement which executes a set of statements if condition used is true.

```javascript
// Block will be executed
if (true){}
// Block will be executed
if (1){}
// Block will be executed
if (""){}
// Block will not be executed
if (false){}
// Block will not be executed
if (nil){}
```

#### `nil`
This keyword represents the absence of a value. If a variable is just declared, and not initialized, it will by default have a `nil` value. You can also explicitly assign a `nil` value to a variable. `nil` will always produce a 'falsey' result when evaluated in a boolean expression.

```javascript
var a;
// nil
print a + "\n";
// explicitly assigning nil
a = nil;
// else block will get executed
if (nil) {}
else {print "nil\n";}
```

#### `or`
It's a binary operator which requires two operands. `or` operator returns first operand if it's truthy, otherwise returns second operand.

```javascript
// Will print 1
print 1 or 2;
// Will print 2
print nil or 2;
// Will print 2
print 2 or nil;
// Will print nil
print nil or nil;
// Will print false
print false or false;
// Will print true
print true or false;
// Will print true
print false or true;
```

#### `print`
This keyword is used to print any value to the console. It expects an expression followed by a semicolon. To move cursor to the next line, you should use "\n" at the end.

```javascript
// Will print true
print true + "\n";
// Will print false
print (5 > 2 != nil) == false + "\n";
```

#### `return`
Used to return value from a function. It takes an expression followed by a semicolon. If it doesn't have an expression, it returns `nil`.

```javascript
fun func(){
    return "Hello\n";
}
// Prints "Hello"
print func();

fun func2(){
    return;
}
// Prints `nil`
print func2();
```

#### `true`
This represents the `true`, a boolean constant. This value is returned when an expression is evaluated as true. All of the following expressions will be evaluated as true, and will print `true`.

```javascript
print nil==nill;
print "\n";
print false==false;
print "\n";
print 6<=10;
print "\n";
```

#### `var`
This keyword is used to declare variables. Since **Rslox** is a dynamically typed langauge, you don't need to specify data type.

```javascript
var name;
{
    // Local scope
    var age;
}
```

#### `while`
This keyword is used to declare `while` loop, which expects an expression in a pair of parenthesis followed by a block. 

```javascript
var i = 1;
while (i <= 5) {
    print i+" ";
    i = i + 1;
}
print "\n";
```