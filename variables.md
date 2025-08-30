## Variables
You can declare variables using the keyword `var`. Since **rslox** is dynamically typed, so you don't need to specify the data type. Data type will be determined on compiled time and runtime. You can declare a variable like this:
```javascript
var age;
``` 
If we initialize it with a value while declaring, we can do so:
```javascript
var age = 22;
```
Since '22' is a constant, and is a number, compiler will treat `age` as a number. Sometimes compiler doesn't know about data type of a variable, it will be resolved at runtime. You can learn more about this in functions.

### Reading variables
You can use variables just like expressions, because they hold some data. If a variable isn't initialized explicitly, they have a `nil` value by default. Variables can be used as a whole expression, as well as a part of a complex expression. So this code is valid
```javascript
age;
```
Since it's an expression statement, and value isn't used anywhere, this value will be ignored. `age` variable can be read in following ways:
```javascript
var age = 22;
// Will print '22'
print age + "\n";
// A relatively complex expression
var message = "Age: " + age + "\n";
// Will print 'Age: 22'
print message;
``` 
and the output will be:
```bash
22
Age: 22
```

Trying to read a variable without declaration, should throw a compiler error.

### Updating variables
Variables in **rslox** are mutable by default. It means that you can declare a variable, and update its value later without any problem. Consider following code
```javascript
// Declare and initialize a global variable
var name = "Ameer";
// Print value on console. '\n' for go to new line.
print name + " ";
// Update the global variable
name = "Hamza";
// Print the updated value on console
print name;
```
You'll see the output on your terminal like this:
```bash
> Ameer Hamza
```

### Assigning a different data type
Since we don't specify the data type while declaring a variable, it can be of any type. If we assign a string value to a variable, there's nothing stopping us from assigning a number or boolean value to the same variable again. So this code is valid:
```javascript
var name = "rslox";
// Will print 'rslox'
print name + "\n";

name = true;
// Will print 'true'
print name + "\n";

name = 22.123;
// Will print '22.123'
print name + "\n";

name = nil;
// Will print 'nil'
print name + "\n";
```
You can use a single variable for different types without any issue. While this offers convenience, it can introduce bugs if not used carefully. 

### Variable scoping

You can declare variables in a global scope and local scope. Global variables are defined at top level, like they aren't declared in any block. All of the previous examples we saw, were global variables. 

#### Global variables

Global variables are declared at top level, and not in any block, they can be used throughout the script. You can declare at the top

If you want to  block, like simple block, function block, loop block or if/else blocks, variable declared in th 