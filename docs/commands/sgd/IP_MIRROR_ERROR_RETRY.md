# ip.mirror.error_retry



This command identifies how many times mirroring is retried when an error occurs.


**Setvar**


To set the required times that mirroring retries when an error occurs:

```
       ! U1 setvar "ip.mirror.error_retry" "value"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.mirror.error_retry"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "ip.mirror.error_retry" "0"

```

When the `setvar` value is set to `"0"`, the `getvar` result is `"0"` .


1270




SGD Network Commands