# FOR Loops



`FOR` loops are an easy way to iterate through a range of values and run a body of code for each value
iterated.


**Format**
```
          FOR <I> = <A> TO <B> [STEP <C>]
          ~~BODY~~
          NEXT <I>
```

**Parameters**
`<I>` = indicates a numeric variable is used. <I> increments each time through the `FOR#LOOP` .

`<A>` = the value assigned to <I> the first time through the loop

`<B>` = the last value through the loop

`<C>` = (Optional) the amount <I> increments each time through the loop

Values of `I` for the following situations:

|Statement|Result|
|---|---|
|`FOR I=1 TO 10`|`{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}`|
|`FOR I=10 TO 1`|`{10, 9, 8, 7, 6, 5, 4, 3, 2, 1}`|
|`FOR I=1 TO 10 STEP 2`|`{1, 3, 5, 7, 9}`|
|`FOR I=10 TO 1 STEP 2`|`{10, 8, 6, 4, 2}`|
|`FOR I=10 TO 1 STEP 2`|`{ } FOR LOOP skipped`|



478


ZBI Commands


**Example**
This is an example of how to use the `FOR LOOP` command:

```
 10 FOR X=1 TO 10 STEP 1
 20 PRINT X; ":ZBI IS FUN"
 30 NEXT X

```

**Comments**
`FOR` loops can be nested but cannot overlap. Variables cannot be reused by the nested loops.


479


ZBI Commands