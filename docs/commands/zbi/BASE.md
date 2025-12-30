# Base Types and Expressions



There are two base types in the ZBI language. These types are Integers and Strings. Integers are whole
numbers that contain no fractional part.


The range of values for integers is: -2,147,483,648 to +2,147,483,647


Strings are character arrays. The string length is only limited by the amount of memory in the system
(version 2.0 and higher). Each character can have a value between 0 and 255 (version 2.0 and higher).


The use of control characters (0-31) may be difficult to debug based on the handling of control characters
in different communications programs. In addition the ETX (3) will terminate a ZBI application when it is
received on the console port. Use the CHR$ function when control characters must be placed into strings.


**NOTE:** In ZBI version 1.4 and lower, there was a string length limit of 255 characters.


This section is organized as follows:


          - Variable Names


          - Variable Declarations


          - Constants


          - Arrays


          - Assignment


          - Numeric Expressions


          - String Concatenation (&)


          - Sub-strings


          - Boolean Expressions


          - Combined Boolean Expressions

## **Variable Names**


To distinguish strings from integers, string variable names must end in a $. Variable names must start
with a letter and can include any sequence of letters, digits, and underscores. Function and command
names must not be used as a variable name. Variable names are not case sensitive and are converted to
uppercase by the interpreter.


A common mistake is to use a command or function name as a variable. To avoid using these reserved
words, ZBI-Developer can be a useful resource. Reserved words are highlighted making it easier to spot
this occurrence and thus, saving debugging time.


**Valid variable names**


I, J, K, VARNAME, VARSTR$, MYSTR$,MY_STR9$


**Invalid Names**


STR$ = Reserved word


ORD = Reserved word


VAL = Reserved word


W# = Invalid character (# )


467


ZBI Commands


9THSTR = Variable can not start with a number

## **Variable Declarations**


ZBI will allow storage of up to 255 variables. If more variables are needed, consider using arrays to store
data. The base array will take up one of the 255 variable slots, but it can be declared to allow for many
indices.


Variables can be declared explicitly or implicitly. If a variable has not been used before, it will be declared
when used. The default value for an integer will be zero and the default value of a string will be an empty
string.


**Explicit**
```
        DECLARE NUMERIC <variable_name>
        DECLARE STRING <variable_name$>
```

If the variable existed before the DECLARE statement, it will be defaulted.


**Implicit**
```
        LET <variable_name> = NUMERIC EXPRESSION
        LET <variable_name$> = STRING EXPRESSION
```

The Interpreter is limited to 255 variables. If more variables are required, consider using arrays.

## **Constants**


Integers are represented simply by numbers, such as 5, -10, 10000. Do not use commas in integer
constants. Strings are enclosed by quotes. If a quote is required in the string, use double quotes, such as
"Look here->""<-" would result in the string – Look here->"<-.

## **Arrays**


An array is a collection of string or integer values used by a program. Array indices are accessed through
parentheses. Array indices start at 1 and end at the length of an array (for example, MyArray(3) returns
the value in the third location of the variable array). One- and two-dimensional arrays are allowed. Twodimensional arrays are referenced with two indices in parentheses, separated by a comma.

Arrays must be allocated through the use of the `DECLARE` command. Arrays can be re-dimensioned by
using `DECLARE`, however, this will replace the original array.

Array size is limited only by the size of the memory available.


**Format**
```
        DECLARE STRING <ARRAYNAME$>(<SIZE>)
        DECLARE STRING <ARRAYNAME$>(<ROWS>,<COLUMNS>)
        DECLARE NUMERIC <ARRAYNAME>(<SIZE>)
        DECLARE NUMERIC <ARRAYNAME>(<ROWS>,<COLUMNS>)
```

**Parameters**
```
        <SIZE> = number of entries in a single dimension array
        <ROWS> = number of rows in a two dimensional array
        <COLUMNS> = number of columns in a two dimensional array

```

468


ZBI Commands


**Example**
An example of `ARRAY` code is:

```
        10 DECLARE STRING INARRAY$(3)
        20 FOR I = 1 TO 3
        30 PRINT "Name "; I; ": ";
        40 INPUT INARRAY$(I)
        50 NEXT I
        60 PRINT INARRAY$(1); ", "; INARRAY$(2); ", and "; INARRAY$(3);
        70 PRINT " went to the park"
        RUN
        Name 1: Jim
        Name 2: Jose
        Name 3: Jack
        Jim, Jose, and Jack went to the park

```

**Comments**
If you attempt to access an array outside of its allocated bounds, an error will occur.

## **Assignment**


All lines must start with a command. In order to assign a value to a variable, use the LET command. Multiple
variables can be placed before the =. The variable types must match the expression type.


The right side of the assignment is always calculated completely before the assignment is made. This
allows a variable to be the target and source of the assignment.


When a value is assigned to a string variable with a sub-string qualifier, it replaces the value of the substring qualifier. The length of the value of the string variable may change as a result of this replacement.


**Example**
An `ASSIGNMENT` example:

```
        10 LET A=5
        20 LET B$="HELLO"
        30 LET B$(5:5)=B$

```

469