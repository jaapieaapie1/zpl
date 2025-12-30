# TRACE




ZBI Commands


This command enables you to debug an application by outputting the executed line numbers and changed
variables to the console.


**Format**
```
   TRACE ON
   TRACE OFF
```

**Parameters**
`<ON/OFF>` = controls whether `TRACE` is active (ON) or disabled (OFF).

If `DEBUG` is activated and the `TRACE` command is on, trace details are displayed. When any
variables are changed, the new value displays as follows:

`<TRACE> Variable` = `New Value`

Every line processed has its line number printed as follows:
```
   <TRACE> Line Number
```

**Example**
An example of TRACE command in use:

```
   10 LET A=5
   20 GOTO 40
   30 PRINT "Error"
   40 PRINT A
   DEBUG ON
   TRACE ON
   RUN
   <TRACE> 10
   <TRACE> A=5
   <TRACE> 20
   <TRACE> 40
   5

```

**Comments**
This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.

It is recommended that you avoid using the `TRACE` command when writing programs in the
environment, instead use the Debug capabilities of ZBI-Developer.
ZBI-Developer


462