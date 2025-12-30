# device.user_p1



This command saves and retrieves user specified values.


**Setvar**


To instruct the printer to set user parameters:

```
       ! U1 setvar "device.user_p1" "value"

```

**Values**


An alphanumeric text string (1 - 20).


**Default**
```
          ""

```

**Getvar**


To retrieve user specified parameters:

```
       ! U1 getvar "device.user_p1"

```

**Example**

This `setvar` example shows the value set to `"test"` .

```
       ! U1 setvar "device.user_p1" "test"

```

When the `setvar` value is set to `"test"`, the `getvar` result is `"test"` .


784


SGD Printer Commands