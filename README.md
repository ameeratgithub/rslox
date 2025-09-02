# Introduction
This is a Rust implementation of *Lox* language explained by *Robert Nystorm* on [Crafting Interpreters](https://craftinginterpreters.com/). It's a compiled language, where compiler is responsible for generating byte code from source code and a virtual machine is responsible for executing that byte code. 

## Getting Started
To get started, you need to download the software from the release page.

### Installation
1. Go to the [latest](https://github.com/ameeratgithub/rslox/releases/tag/v0.1.1) release page.
2. Download the binary for your operating system
3. For windows
   1. Double click on the installer and follow the instructions. 
   2. After installation, close the terminal/powershell/cmd, if they're open.
   3. Open cmd/powershell/terminal
   4. Type `rslox --version`
   5. If it's correctly installed, it will show `rslox 0.1.1`.
4. For linux
   1. Assuming you've downloaded the binary, you have to move it to `/home/your-username/.local/bin/` directory.
   2. If .local/bin doesn't exist, execute this command: `mkdir -p ~/.local/bin`
   3. Then enter this command. `mv /path/to/rslox /home/your-username/.local/bin`
   4. Restart your terminal.
   5. Type `rslox --version`.
   6. If everything is correctly done, it will show `rslox 0.1.1`

### Running Programs

If your code is written in a file, you can specify the file path like this

```bash
rslox --file="fibonacci.lox"
```

There are many code examples given in the `lox` directory. You can run these examples, tweak around and push the limit of the compiler and virtual machine.

If you want to just check syntax quickly, type `rslox` in your terminal and press enter. It will take you to REPL environment, where you can test commands like these

```bash
> println("Hello" + " " +"World!"); 
``` 

### Features

Since compiler and virtual machine is bundled as one software package, you don't need to worry about binary files being generated. Virtual Machine automatically takes binary from compiler and starts executing bytecode, when compilation completes.

This language supports a lot of features like many functional programming languages. You can:

- Use 4 different data types: Numbers, Strings, Booleans and `nil`.
- Declare variables
- Evaluate complex expressions using arithmetic, logical, comparison and assignment operators
- Implement control flow logic using `if`-`else`, and `for` and `while` loops.
- Define custom functions for reusability.
- Use native functions, `clock()`, `println()` and `print` statement.

### Syntax
Details about language syntax are provided in `docs` directory, present in the repo's root directory. Refer to the following links if you want to learn how to use language syntax.

- [Data Types](https://github.com/ameeratgithub/rslox/blob/main/docs/data_types.md)
- [Variables] (https://github.com/ameeratgithub/rslox/blob/main/docs/variables.md)
- [Expressions] (https://github.com/ameeratgithub/rslox/blob/main/docs/expressions.md)
- [Control Flow](https://github.com/ameeratgithub/rslox/blob/main/docs/control_flow.md)
- [Functions] (https://github.com/ameeratgithub/rslox/blob/main/docs/functions.md)
- [All Keywords](https://github.com/ameeratgithub/rslox/blob/main/docs/keywords.md)

### Roadmap
Following features are planned to be added in upcoming weeks.

1. Closures [deadline 15-09-2025]
2. Sophisticated Garbage Collector [deadline 25-09-2025]
3. Classes [deadline 05-10-2025]
4. Inheritance [deadline 15-10-2025]
5. Optimization [deadline 25-10-2025]

Other than these features, I plan to fix bugs and add utility features/functions for better experience. 

## Contribution
If you encounter any issue, please feel free to create a new issue. If you want to contribute, feel free to create a PR.