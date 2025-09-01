## Expressions
Expressions are always evaluated to a single value. Operations have precedence over one another and would be executed in order which make sense. There are multiple kind of operations which can form complex expressions.

- Arithmetic Operations
- Logical Operations
- Comparison Operations
- Assignment Operations

### Arithmetic Operations
Arithmetic operations include addition, subtraction, multiplication, division and negation. Addition (+) can be performed on both strings and numbers. Remaining operations are only for numbers. 

In an expression, multiplication and division tale precedence over subtraction and addition. When you type an expression like below, obviously multiplication or division will be evaluated first.

```javascript
print 5 + 3 * 6 / 3;
```
In this case, '11' gets printed. Compiler will not evaluate `5 + 3`, because its precedence is lower than '\*'. When it encounters '\*', it will keep moving forward until the end of the expression, or until it finds something which has lower precedence. Next to '\*', there is division, which is of same precedence. So,  `6 / 3` will get evaluated first, and result will be '2'. Then '2' will be multiplied with '3'. At last, '6' will be added to '5' and result will be '11'.

There is also a negation operator, '!', which is a unary operator. Unary operators should be evaluated before all other arithmetic, logical, comparison and assignment operations.

Let's say negation (!) operator doesn't take precendence over multiplication, this would produce an error, because '\*' expects a numeric operand on the right side, but here compiler is encountering a '-' sign. It is not an invalid expression, which should produce 12 as a result.

```javascript
print -4 * -3;
```

### Logical Operations

Logical operations include `and` and `or` keywords. `and` should be evaluated to true if both values are true, otherwise it will be evaluated as false. Or will be evaluated as false if both values are false, otherwise it will be evaluated as true. Logical operations don't take precedence over arithmetic or comparison operations, but they take precedence over assignment operations. 

Examples below are are great demonstration of operator precedence.

```javascript
// true
print 4 * 3 < 5 + 8 and 5 > 3 ;
// false
print 4 * 3 < 5 + 8 and 5 < 3 ;
// true
print 4 * 3 < 5 + 8 or 5 < 3 ;
// false
print 4 * 3 > 5 + 8 or 5 < 3 ;

```

- First print statement should be print 'true', because 12 is less than 13 AND 5 is greater than 3. 
- Second statement should print false because right operand of `and` is false.
- Third statement should print true because left expression of `or` is true.
- Fourth statement should print false, because both operands of `or` are false. 

### Comparison Operations
Comparison operations support following operators, `>`, `>=`, `<`, `<=`, `==`, `!=`. Operators `==` and `!=` are also known as equality operators and have lower precedence than other comparison operators. To understand their precedence importance, let's consider the following expression:
```javascript
print 9 >= 5 == 12 <= 100;
```
It should print 'true'. 9 is greater than 5, and 12 is less than 100. Both are true, and 'true' is equal to 'true', so 'true' will get printed. Suppose `==` doesn't have precedence over `>=` or `<=` operators, one of the following will happen:
- `5 == 12` will return false, which will make the expression like this: `9 >= false <= 100`, which obviously doesn't make sense.
- `12 <= 100` will return true, which will make the expression like this: `9 >= 5 == true`, which is again incorrect.
- `9 >= 5` will return true, which will make the expression: `true == 12 <= 100`.

All of the above cases produce invalid output, but our expression is prefectly valid and should return 'true'.

### Assignment operations
Only one assignment operation is currently supported, which is `=`. So when you declare a variable you can assign a value, an expression, another variable or even a function. So all of examples in code below are prefectly valid and should produce correct result

```javascript
var technology = "Blockchain";
var career;
career = "Computer Science";

var web3 = technology;

fun printHello(){
    print "Hello\n";
}

var function = printHello;
function();
```

There are another kind of unsupoorted assignment and increment operators commonly used in loops. These include `++`, `--`, `+=`, `-=`, `*=`, `/=`. These prevent you from writing variable name twice. To increment you have to do something like this:

```javascript
var a = 10;
a = a + 20;
a = a + 1;
```