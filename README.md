# TestLang

A test programming language designed to demonstrate language-building capacities.

## Features

The main feature of TestLang is its simplicity. There is only one way to do any one thing. This prevents unreadable code.

In TestLang, regular functions and user-defined functions are not able to modify the state of variables. This prevents side-effects and gives a better programmer experience.

TestLang also emphasizes quick and easy development. Program refinement is done iteratively, through trial and error. This gives developers more options for quickly prototyping and adds to the language experience as a whole.

Readability is key to TestLang's nature. You can put external commands inside of function definitions, allowing developers to see what they need to without going elsewhere in code.

## Syntax

### Variables

To create a variable, use the `var {name}` notation:

```testlang
var myVar
```

This will create a variable, but will not assign a value to it. To assign a value to a variable, use the `var {name} {value}` syntax:

```testlang
var myVar 42
```

Internally, variables are stored as strings. However, your variable can be representative of anything. Booleans are either `yes` or `no` and are also internally represented by strings.

To set a variable to a specific value, you can set the interpreter to Assignment Mode. The next instruction executed will be assigned to the variable:

```testlang
var myVar

## Set interpreter into assignment mode for `myVar`
= myVar

## Add two numbers together, putting the result into myVar
add 2 3

print myVar
## => 5
```

### Functions

You can define a function using the `def` keyword. To end a function definition and register the function, use the `fed` keyword.
Operations inside a function are achieved using the `|` symbol. Omitting this symbol will cause the code to be executed in the main loop. This enables global variable management inside functions without compromising on readability.

```testlang
def sayHello
| print Hi there!
fed
```

To call a function later on, simply call it by name:

```testlang
sayHello
## => Hi there!
```

You can have arguments in your functions. Just say what their name is.

```testlang
def sayHello name
| print Hi,
| println name
fed

sayHello MysteriousOyster
## => Hi, MysteriousOyster
```

Functions return whatever their last executed command was.

```testlang
def fancyAdd x y
| print Adding
| print x
| print and
| println y

| add x y
fed

var return
= return
fancyAdd 1 2

print return
## => Adding 1 and 2
## => 3
```

### Comments

As has already been shown, comments are denoted by two hashtags `##` at the start of a line. To prevent large comments and confusing blockages, TestLang doesn't have multiline comments. TestLang doesn't support comments hanging onto the end of lines, meaning it forces you to comment in a predictable way.

```testlang
## This function says hello to you
def sayHello
| print Hello! ## NOT ALLOWED
fed

## This is the preffered way
## to comment on multiple lines
##
## Leave a commented space for
## page breaks.
```

### Control Flow

TestLang has one predictable way to control the flow of a program: if blocks. Do define an if block, use `bif` (short for "begin if"). End an if with `eif` (short for "end if").

The greater-than symbol `>` is used to denote a command inside of an if block. Similar to functions, external commands can be called from inside the if block for greater readability. However, keep in mind these external commands only execute if the condition is true, and also execute before any commands inside the if block.

After the `bif` statement, either put a boolean variable or create a newline and an expression to be evaluated to `yes` or `no`.

```testlang
var isTestLangAGoodProgrammingLanguage yes

bif isTestLangAGoodProgrammingLanguage
> println Yeah!
eif
## => Yeah!

```

### Importing

To import other libraries, such as the common tllib, use the keyword `imp` along with whatever path you are importing from.

```testlang
imp tllib/math.tl

var returnValue
= returnValue
math.mult 4 3

bif
eq returnValue 12
> println Math isn't broken.
eif
```

## Inbuilt Functions

TestLang has a few inbuilt functions. These provide the building blocks for complex operations.

- `add <x> <y>`: Add two numbers together. To subtract, add a negative number.
- `!! <bool>`: Takes the opposite of the provided boolean
- `&& <bool_x> <bool_y>`: Performs an AND operation on two bools
- `|| <bool_x> <bool_y>`: Performs an OR operation on two bools
- `eq <x> <y>`: Returns whether two variables are equal
- `gt <x> <y>`: Whether x > y
- `lt <x> <y>`: Whether x < y
- `ech <x>`: Echoes the value of x back. Useful for returning from if statements or recursions.
