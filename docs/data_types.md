## Data types
**Rslox** currently supports 4 datatypes: number, string, boolean and 'nil'. You don't have to mention datatype explicitly. You can just assign the value to variable, it will automatically be detected.

### Number
Numbers in 'Rslox' are basically `f64` values, a 64-bit floating point number. It takes 8 bytes of memory. It can store a value from `-1.7976931348623157e+308` to `1.7976931348623157e+308`.

You can use number datatype like this:

```javascript
var a = 10;
var b = 5;

// 15
print a + b + "\n";
// 5
print a - b + "\n";
// 50
print a * b + "\n";
// 2
print a / b + "\n";
// -10
print -a + "\n";
```

You can't divide anything by zero. You will get a 'inf' as a result, which indicates infinity. If you do `0/0`, it will return 'NaN'. 

### Boolean
Boolean variables indicate that value is either 'true' or 'false'. If value doesn't exist for any type, it's `nil` by default. Booleans are useful for control flow and deciding if condition is correct or not. Let's extend the previous example with comparison operators.

```javascript
var a = 10;
var b = 5;
// true
print (a > b) + "\n";
// false
print (a < b) + "\n";
// true
print (a >= b) + "\n";
// false
print (a <= b) + "\n";
// false
print (a == b) + "\n";
```

Note that you need to put parenthesis around the comparison expressions, because '+' operator takes precedence over other comparison operators. If we do something like this, `a > b + "\n"`, `b + "\n"` will get evaluated first, and then will get compared later with `a`. Since `b + "\n"` is now `"5\n"`, which is a string, it will get compared to `a` like `10 > "5\n"`. It doesn't make sense, so you'll get a runtime error. 

```bash
Runtime Error: Invalid operation on these operands.
```

### Nil
It represents an absence of a value. When you declare a variable, but don't initialize it, it will have `nil` by default. It's the alternative to `null`, which is present in many programming languages. It's a keyword, so you can also explicitly assign it to the variable. 

```javascript
var name;
// nil
print name + "\n";

name = "Rust";
// Rust
print name +"\n";

name = nil;
// nil
print name +"\n";
```

If `nil` value is used in a condition, it will be considered as `false`.

### Strings
Strings are just collection of characters allocated on heap. You can make a string by putting double qoutes`"` around the value. 

```javascript
// Is a number
var number = 123.123;
// Is a string
var number_str = "123.123";
// Is a nil value
var nil_val = nil;
// Is a string
var nil_str = "nil";
// Is a boolean value
var bool = true;
// Is a string
var bool_str = "true";
```

You can concatenate any value with a string.

```javascript
var number = 2.12;
// Number: 2.12
print "Number: " + number + "\n";
// True: true
print "True: " + true + "\n";
// Nil: nil
print "Nil: " + nil + "\n";
// Nil: nil
print "String: " + "A String" + "\n";

fun function(){
    return "Hello";
}
// Function: <fn function>
print "Function: " + function + "\n";
// Native Function: <native>
print "Native Function: " + clock + "\n";
```

You see that custom function and native function (clock) are not being called, they're just objects, only their string representation is displayed. 
 