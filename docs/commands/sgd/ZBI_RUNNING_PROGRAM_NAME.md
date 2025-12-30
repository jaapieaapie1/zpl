# zbi.running_program_name



This command identifies the name of the ZBI 2.0 program that is currently running.


**Getvar**


To retrieve the name of the currently running ZBI 2.0 program:

```
       ! U1 getvar "zbi.running_program_name"

```

**Example**

In this example, the `getvar` command causes the printer to respond that the program `choices.bas` is
currently running.

```
       ! U1 getvar "zbi.running_program_name"
       "CHOICES.BAS"

```

1064


SGD Printer Commands