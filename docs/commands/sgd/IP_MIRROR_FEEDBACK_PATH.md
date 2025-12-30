# ip.mirror.feedback.path



This command identifies where the feedback file is stored on the mirroring server.


**Setvar**


To set the path on the mirroring server that stores the feedback file:

```
       ! U1 setvar "ip.mirror.feedback.path" "value"

```

**Values**


Alphanumeric text (1 to 50 characters)


**Default**
```
          "Zebra/feedback"

```

**Getvar**


To retrieve the path on the mirroring sever that the printer is currently using to store the feedback file:

```
       ! U1 getvar "ip.mirror.feedback.path"

```

**Example**

This `setvar` example shows the value set to `"Zebra/feedback"` .

```
       ! U1 setvar "ip.mirror.feedback.path" "Zebra/feedback"

```

When the `setvar` value is set to `"Zebra/feedback"`, the `getvar` result is `"Zebra/feedback"` .


1274


SGD Network Commands