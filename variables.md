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

Trying to read a variable without declaration should throw a compiler error. If you fire up REPL and quickly type following code
```javascript
print a;
```

You will see the following error:
```bash
Runtime Error: Undefined variable 'a'
[line 1] in <script>
```

There is another interesting case when you try to assign variable itself while declaring. It shouldn't be valid. Consider this:
```javascript
var a = a;
```
`a` just got declared and shouldn't be assigned to itself. Doing so will throw an error. The error will be different for local variables and global variables. 
For global variables, consider this:
```javascript
var a = 10;
var a = a;
print a + "\n";
var b = b;
print a + "\n";
```
Since `b` isn't already defined, you'll see the error
```bash
Undefined variable 'b'
```
Because global variables are allowed to be re-declared, `var a=a;` is valid.

For local variables, the case is a bit different. You can't re-declare a variable, so when you do something like this:
```javascript
{
    var name=name;
}
``` 
You'll get the error:
```bash
[line 2] Error at 'name': Can't read local variable in its own initializer
```

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
Global variables are declared at top level, and they can be used throughout the script. They are accessible in functions, control flow statements and simple blocks. Blocks can update global variables and changes will be reflected throughout the script. Consider following example:

```javascript
var age=22;
// Will print '22'
print age+"\n";

{
    age= 23;
}
// Will print 23
print age+"\n";

if (true){
    age = 26;
}
// Will print 26
print age+"\n";
```

If you want to re-declare a global variable, it will not throw an error. Like this example is completely valid, and should print '20' on the console.
```javascript
var a=10;
var a=20;
print a;
```
This is known as 'shadowing'. It's important for REPL where you can re-declare variables with the same name. Note that this is only allowed for global variables. Local variables don't support shadowing, and will throw an error.

#### Local variables
Variables defined in a block are local to that block. They can't be accessed outside the block. Let's look at following example.
```javascript
{
    var language = "Rust";
    print language + "\n";
}
print language + "\n";
```

You'll see output like this

```bash
Rust
Undefined variable 'language'
[line 1] in <script>
```

In the block, the variable is available, hence you can access it. But outside the block, variable is unknown. 

As described earlier, you can't re-declare the variable in local scope. If you do something like this:
```javascript
{
    var grade;
    var grade;
}
```

You'll get the following error.
```bash
Compiler Error: [line 3] Error at 'grade': Already a variable with this name in this scope.
```