# head.element_test



This command will cause the printer to immediately run the head test on all printhead elements. The
command can also display the result of the last head element test.


**Do**


To run the head test on all printhead elements:

```
       ! U1 do "head.element_test" ""

```

**Getvar**


To display the result of the last head element test:

```
       ! U1 getvar "head.element_test"

```

**Values**


The possible getvar responses include:

              - `"Head Elements OK"` All head elements passed the test.

              - `"n, n..."` A comma-separated list of elements that failed the test.

              - `"Initialization Failed"` The test could not start.

              - `"Failed to Attach"` The test could not start.

              - `"Please Run Test"` Default response if there are no results to display.

              - `"In Progress"` The element test has been started but not completed yet.

**Default**
```
          "Please Run Test"

```

**Example**


This example shows a single element that failed the test.

```
       "86"

```

This example shows a list of elements that failed the test.

```
       "75,309,456,778,779"

```

832


SGD Printer Commands