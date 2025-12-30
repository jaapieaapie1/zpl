# ip.mirror.feedback.freq



This command specifies the time interval (in minutes) between performing feedback file uploads.


**Setvar**


To set the number of minutes to wait between feedback file uploads:

```
       ! U1 setvar "ip.mirror.feedback.freq" "value"

```

**Values**


A numeric value (0 - 65535)


**Default**
```
          "0"

```

**Getvar**


To retrieve the number of minutes set to wait between feedback file uploads:

```
       ! U1 getvar "ip.mirror.feedback.freq"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "ip.mirror.feedback.freq" "0"

```

When the `setvar` value is set to `"0"`, the `getvar` result is `"0"` .


1272


SGD Network Commands