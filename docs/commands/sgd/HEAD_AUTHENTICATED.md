# head.authenticated



This command reports if the printhead is authenticated.


**Getvar**


To return the current state of the authenticated printhead:

```
       ! U1 getvar "head.authenticated"

```

**Result**

`"yes"` means the printhead has passed printhead authentication

`"no"` means the printhead has failed printhead authentication


**Example**


In the example below, the getvar returns the current state of the authenticated printhead.

```
       ! U1 getvar "head.authenticated" "yes"

```

829


SGD Printer Commands