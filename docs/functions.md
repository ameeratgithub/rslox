## Functions
Functions are important for code reuse and help prevent declaring everything globally. One feature shouldn't interfere with others. This makes it easier to add features to the software.

### Usage
Functions are pretty simple to use in **Rslox**. To declare a function, `fun` keyword is used, followed by function name with parenthesis, '()'. Function body starts with left curly brace, '{', and ends with right curly brace, '}'.   

Complete syntax looks like this:

```javascript
fun functionName() {
    print "Well, hellooooo...\n";
}
```

What's the use of a function if you can't call it? You can do so by writing function name followed by parenthesis. And of course you have to put semicolon at the end.

```javascript
functionName();
```

It will print "Well, hellooooo..." on the console.

Function names are also like variable names. A valid variable name is also valid function name. An invalid variable name is also an invalid function name. You can't use reserved keywords as variable names or function names.

#### Function Parameters

If we're talking about reusability, a function isn't much better if it can't accept parameters. Specifying parameters is simple and doesn't require `var` keyword.

```javascript
fun add(a, b) {
    print a + b + "\n";
}
add(10, 20);
```

Above code should print '30' on the console. Parameters `a` and `b` are declared as local variables to that function, and should not be accessed outside the function body.

#### Return Statement

If you want to return something from function, `return` keyword is used. `return` keyword expects the value, whether it is a constant or a variable, or a complex expression. 

```javascript
fun add(a, b){
    return a + b;
}
print add(10, 20) +"\n";
```

A bit complex example demonstrates scoping and assignment of local variables.

```javascript
fun getGradeFromMarks(marks){
    var grade;
    // else if clause is not currently supported
    if (marks >= 90){
        grade = "A+ Grade";
    }
    else{
        if (marks >= 85 and marks < 90){
            grade = "A Grade";
        } else{
            grade = "You're not in top 2 grades";
        }
    }
    return grade;
}

print getGradeFromMarks(90) + "\n";
print getGradeFromMarks(85) + "\n";
print getGradeFromMarks(84) + "\n";
```

### Native Functions
**Rslox** also support native functions, which are pretty much easier to add. Currently two native functions are supported, which are `clock()` and `println()`. 

#### `clock()`
`clock()` returns time in seconds as 64 bit floating point number. You can measure performance of the code by using this function. Following example calculates Fibonacci number and also measures how long does it take.

```javascript
fun fib(n) {
  if (n < 2) return n;
  return fib(n - 2) + fib(n - 1);
}

var start = clock();
print fib(35) + "\n";

var end = clock() - start;
print "Time in seconds: " + end + "\n";
```

Example above can take over a minute, depending on system specifications. It's not a great way to calculate Fibonacci numbers, but this example tests the language strength by pushing its limits. 

#### `println()`
This is a utility function which is similar to print statement, but appends a new line character, '\n', at the end. 

```javascript
println("Well, Hellooo...");
```

If no value is passed, it just prints a new line. Please note that, passing functions directly is not supported right now, and will require more work. So instead of doing something like

```javascript
fun getName(){
    return "Your Name";
}
// This will cause panic because stack is corrupted
println(getName());
```

You can store the result in a variable and then pass the result to `println()`, like this:
```javascript
fun getName(){
    return "Your Name";
}
var name = getName();
println(name);
```