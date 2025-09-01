## Control flow
What even is a program if you can't include logic and take decisions based on different conditions. **Rslox** supports if_else, for loop and while loop. These are enough for a lot of use cases. 

### If conditions
Like a lot of programming languages, `if` statement takes an expression and if the value is `true`, will execute the 'then' block.

`if` statements start with, well, `if` keyword, needs an expression to evaluate, followed by a block. Expression should be in a pair of open and close parenthesis `()`. If any of these requirements are not, a syntax error will occur. 

If you add `else` statement, `else` will be executed when `if` expression evaluates to `false`. 

Following example illustrates the use of `if` statements.

```javascript
var should_print = true;
if (s){
    print "Value is true";
}else{
    print "Value is false";
}
```

"Value is true" should get printed. You can directly place `true` or `false` keywords, instead of using variable name as an expression. 

```javascript
if (false){
    print "Value is true";
}else{
    print "Value is false";
}
```
"Value is false" should get printed.

An expression can be 'truthy' or 'falsey'. It means if a value has `nil`, it means it has no value, it would be evaluated as 'falsey'. Consider this example:

```javascript
var name;
if (name){
    print "Name is "+ name;
}else{
    print "Name is not provided.";
}
```

`name` by default has `nil` value, which be considered as false when used in if_else or loops. Above example will print following message on the console.

```bash
Name is not provided.
```

If you assign a value to `name`, value will get printed. Apart from `nil` and `false`, every value gets treated as truthy value, even when the value is '0' or an empty string. In following example, every if condition should be evaluated to true.

```javascript
var value=0;
if (value){
    // Value:0
    print "Value:" + value + "\n";
}

var value=123.123;
if (value){
    // Value:123.123
    print "Value:" + value + "\n";
}

value = "";
if (value){
    // Value:
    print "Value:" + value + "\n";
}

value = "A String";
if (value){
    // Value:A String
    print "Value:" + value + "\n";
}

value = clock;
if (value){
    // Value:<native>
    print "Value:" + value + "\n";
}

fun printName(){
    // Code here
}
value = printName;
if (value){
    // Value:<fn printName>
    print "Value:" + value + "\n";
}
```

You see it's even true when you assign a function, even build-in function, to a variable. Other than `nil` and `false`, everything should be evaluated as truthy value.

### Loops
For repetition and control flow, **Rslox** supports `for` loop and `while` loop. These are simple loops and has syntax like javascript. 

#### While loop
While loop starts with the keyword `while`. It expects an expression to be evaluated in a pair of parenthesis, followed by a block. Loop will continue to execute block until condition is false. Following example just prints numbers from 1-5. If number is greater than 5, while loop will exit.

```javascript
var j = 1;
while (j <= 5){
    print j + " ";
    j = j + 1;
}
print "\n";
```

As discussed earlier, while loop also work on 'truthy' values. Until value is 'falsey', loop will continue to execute. When value of expression being evaluated is 'falsey', loop will exit. This example isn't best and you shouldn't use variables in a loop like this, but this code is correct and will print the `str` only once.

```javascript
var str = "Blockchain";
while (str) {
    print str + "\n";
    str = nil;
}
```
On first iteration, `str` has truthy value, a string. `while` will consider it as true and control will enter the loop body. `str` will get printed and then `str` will be assigned `nil`, a falsey value. When control goes back to evaluate the `str` variable, it will detect that value is now 'falsey', loop will exit.

#### For loop
For loop in **Rslox** resembles for loop in many languages. It starts with `for` keyword, expects 3 statements in a pair of parenthesis and a body. First statement is variable declaration and initialization. Second is where you check if condition is true, and whether loop should continue or exit. Third statement is where you update your value.

```javascript
for (declaration;condition_check;value_update){
// Body
}
```

Following example will print numbers from 0-4, each at a new line.

```javascript
for (var i=0; i<5; i=i+1){
    print "Index: "+ i + "\n";
}
```

These 3 statements can be omitted, or a changed a bit. For example you can declare variable in for loop, or make it outside the loop like this:

```javascript
var i;
for (i=0;i<5;i=i+1){
    // 
}
```

Or completely omitting from first statement is also valid. 

```javascript
var i = 0;
for (;i<5;i=i+1){
    //
}
```

Last statement can also be emitted if you can update the state in body, like this

```javascript
var i = 0;
for (;i<5;){
    //
    i = i+1;
}
```

But eliminating middle statement, which is a condition check, will make the for loop run forever. Since **Rslox** doesn't currently support the `break` keyword, there's no way to exit the loop. That's why middle statement, the condition check, is important. 

Also note that removing a statement doesn't mean you have to remove the semicolon. It's the semicolon which distiguish among the statements and is important for the control flow logic. 