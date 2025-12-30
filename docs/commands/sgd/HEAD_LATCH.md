# head.latch



This command identifies if the printhead is open or closed.


**Getvar**


To retrieve the status of the printhead, open or closed:

```
       ! U1 getvar "head.latch"

```

**Values**

`"ok"` is closed

`"open"` is open


**Example**

In this example, the `getvar` retrieves the status of the print head.

```
       ! U1 getvar "head.latch"
       "ok"

```

833


SGD Printer Commands