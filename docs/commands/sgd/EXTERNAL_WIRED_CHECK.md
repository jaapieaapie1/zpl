# external_wired.check



This command controls whether to check for external print server during the network interface search.


**Setvar**


To instruct the printer to set the network interface search:

```
       ! U1 setvar "external_wired.check" "value"

```

**Values**

              - `"on"` = turn on external wired network interface search

              - `"off"` = turn off external wired network interface search

**Default**

              - `"on"` = If wireless option board is not installed

              - `"off"` = If wireless option board is installed


**Getvar**


To retrieve the status of the network interface search:

```
       ! U1 getvar "external_wired.check"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "external_wired.check" "off"

```

When the `setvar` value is set to `"off"`, the `getvar` result is `"off"` .


1115


SGD Network Commands