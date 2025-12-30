# LET




ZBI Commands


The LET command is used to assign value to a specific variable. The expression is evaluated and assigned
to each variable in the variable list.


**Format**
```
   LET <variable> [,<variable>]* = <expression>
```

The variable types must match the expression type or an error message will be displayed.
```
   Error: Poorly formed expression.
```

When a value is assigned to a string variable with a sub-string qualifier, it replaces the value of the
sub-string qualifier. The length of the value of the string variable may change as a result of this
replacement.


**Parameters**
N/A

**Example**
This is an example of how to use the `LET` command:

```
   10 LET A$= "1234"
   15 LET A$(2:3)= "55" ! A$ NOW = 1554
   20 LET A$(2:3)= "" ! A$ NOW = 14

   10 LET A$= "1234"
   15 LET A$(2:3)= A$(1:2) ! A$ NOW = 1124

   10 LET A$= "1234"
   15 LET A$(2:1)= "5" ! A$ NOW = 15234

```

**Comments**
This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


## **Numeric Expressions**

A base numerical expression can be either a constant, variable, or another numerical expression enclosed
in parentheses. The five types used (addition, subtraction, multiplication, division, and exponentiation) are
listed below. When evaluating an expression exceeding the maximum or minimum values at any point
creates an undefined result. (maximum value: 2,147,487,647; minimum value: -2,147,483,648)


Floating point is not supported.

When using division, the number is always rounded down. For example, 5/2=2. Use `MOD` to determine the
remainder.


**Format**


**1.** + (addition) Addition expressions use this format:
```
       <A>+<B>
```

5+2 result = 7


VAL ("25") +2 result =27


470




ZBI Commands


**2.**       - (subtraction) Subtraction expressions use this format:
```
       <A>-<B>
```

5-2 result = 3


VAL ("25") -2 result =23


**3.**       - (multiplication) Multiplication expressions use this format:
```
       <A>*<B>
```

5*2 result = 10


VAL ("25") *2 result =50


**4.** / (division) Division expressions use this format:
```
       <A>/<B>
```

5/2 result = 2


VAL ("25") /2 result =12


**5.** ^ (exponentiation) Exponentiation expressions use this format:
```
       <A>^<B>
```

5^2 result = 25


VAL ("25") ^2 result =625


**Order of Precedence**


In mathematics, the order of precedence describes the sequence that items in an expression are
processed. All expressions have a predefined order of precedence.


The order of precedence is listed below:


**1.** Functions


**2.** Parenthetical Expressions ()


**3.** ^


**4.**       - and /


**5.** + and 

The * and / have the same precedence, and the + and - have the same precedence. Items with the same
order of precedence are processed from left to right.


For example, this expression 5+(8+2)/5 is processed as 8+2=10, followed by 10/5=2, then 5+2 to give a
result of 7.


Functions and parenthetical expressions always have the highest order of precedence, meaning that they
are processed first.

## **String Concatenation (&)**


The basic string expression may be either a constant or a variable, and concatenation (&) is supported.
Using the concatenation operator (&) adds the second string to the first string.
```
     <A$> & <B$>

```

471


ZBI Commands


**Example**
This is an example of how to use the STRING CONCATENATION (&) command:

```
        10 LET A$= "ZBI-"
        20 LET B$= "Programming"
        30 LET C$= A$ & B$
        40 PRINT C$
        RUN
        ZBI-Programming

## **Sub-strings**

```

Using a sub-string operator on a string allows a specific portion of the string to be accessed. This portion
may be the target of an assignment operation or a reference to a portion of the string. To determine the
coordinates of the string portion to be used, count the characters from the beginning to the end of the
string, including spaces.


**Format**
```
        LET <STRVAR$>(<A>:<B>)=<C$>
```

`LET` <C$> = <STRVAR$>(<A>:<B>)

**Parameters**

`<A>` = the position of the first character in the desired string

`<B>` = the position of the last character in the desired string.

`<STRVAR$>` = base string variable

If the A parameter is less than 1, it is automatically assigned a value of 1. Because the string is
calculated starting with 1, the A parameter cannot be less than 1.


If B is greater than the length of the string, it is replaced with the length of the string.


If A is greater than B, a NULL string (""), which points to the location of the smaller of A or the end
of the string, is returned. This is used when adding a string in the middle of another string without
removing a character.


472


ZBI Commands


**Example**
This is an example of a sub-string reference:

```
        LET A$="Zebra Quality Printers"
        LET B$=A$(1:13)
        PRINT B$
        Zebra Quality

```

This is an example of a sub-string assignment.

```
        LET A$= "1234"
        LET A$(2:3)= "55" ! A$ NOW = 1554
        LET A$(2:3)= "" ! A$ NOW = 14

        LET A$= "1234"
        LET A$(2:3)= A$(1:2) ! A$ NOW = 1124

        LET A$= "1234"
        LET A$(2:1)= "5" ! A$ NOW = 15234

```

The best way to think of assignment to a sub-string is as follows: an assignment is like selecting a
word, and pasting over the selection with the new string.

## **Boolean Expressions**


A Boolean expression holds 0 (zero) as false and non-zero as true.


**Formats**

```
        <STRING EXPRESSION> <BOOLEAN COMPARE> <STRING EXPRESSION>
        <NUMERIC EXPRESSION> <BOOLEAN COMPARE> <NUMERIC EXPRESSION>
        NOT(<BOOLEAN EXPRESSION>)

```

**Parameters**
`<STRING EXPRESSION>` = a string variable, string constant or any combination with
concatenation

`<NUMERIC EXPRESSION>` = any mathematical operation


473


ZBI Commands


**Comments**
A numeric expression cannot be compared to a string expression.


Numeric expressions can substitute a Boolean expression where a value of 0 (zero) represents
false and a non-zero value represents true.


Base Boolean expressions:


**Table 15** < (less than)

|Expression|Result|
|---|---|
|1< 2|true|
|2<2|false|
|2<1|false|



**Table 16** <= (less than or equal to)

|Expression|Result|
|---|---|
|1<=2|true|
|2<=2|true|
|2<=1|false|



**Table 17**   - (greater than)

|Expression|Result|
|---|---|
|1> 2|false|
|2>2|false|
|2>1|true|



**Table 18** >= (greater than or equal to)

|Expression|Result|
|---|---|
|1>=2|false|
|2>=2|true|
|2>=1|true|



**Table 19** = (equal to)


|Expression|Result|
|---|---|
|1=2|false|
|2=2|true|
|"A"="AA"|false|
|"A"="A"|true|



**Table 20** <> (not equal to)








|474<br>Expression|Result|
|---|---|
|1<>2|true|


ZBI Commands

## **Combined Boolean Expressions**


`AND`, `OR`, and `NOT` can be used in conjunction with base Boolean expressions to recreate expanded
Boolean expressions.


**Table 21** `NOT`       - Negate the target expression.

|Expression|Result|
|---|---|
|NOT 1=2|true|
|NOT 1=1|false|



**Table 22** `AND`       - Both expressions must be true for a true result.

|Expression|Result|
|---|---|
|1=2 AND 1=2|false|
|2=2 AND 1=2|false|
|1=2 AND 2=2|false|
|2=2 AND 2=2|true|



**Table 23** OR — If either expression is true, the result will be true.

|Expression|Result|
|---|---|
|1=2 OR 1=2|false|
|1=2 OR 2=2|true|
|2=2 OR 1=2|true|
|2=2 OR 2=2|true|



**Order of Precedence**


The order of precedence is listed below:


**1.** Expressions and Functions


**2.** Parenthetical expressions ()


**3.** <, <=, <>, =, =>, >


**4.** NOT, AND, OR


475


ZBI Commands