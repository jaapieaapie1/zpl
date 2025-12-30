# zbi.state




SGD Printer Commands


This command shows the current state of the ZBI 2.0 program.


**Getvar**


To retrieve the current state of ZBI:

```
! U1 getvar "zbi.state"

```

**Values**

    - `"running"` ZBI Interpreter is active and running a program

    - `"off"` ZBI Interpreter is inactive

    - `"stopped"` ZBI Interpreter is active but not executing a program


**Example**

In this example, the `getvar` shows that state of ZBI.

```
! U1 getvar "zbi.state"
"running"

```

1068


SGD Printer Commands