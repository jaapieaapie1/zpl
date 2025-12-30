# ip.mirror.path



This command identifies the base path on the FTP server where the mirror directory resides.


**Setvar**


To set the base path on the FTP server where the mirror directory resides:

```
       ! U1 setvar "ip.mirror.path" "value"

```

**Values**


Alphanumeric text string (1 to 50 characters)


**Default**
```
          "zebra"

```

**Getvar**


To retrieve the base path of the FTP server where the mirror directory resides:

```
       ! U1 getvar "ip.mirror.path"

```

**Example**

This `setvar` example shows the value set to `"zebra"` .

```
       ! U1 setvar "ip.mirror.path" "zebra"

```

When the `setvar` value is set to `"zebra"`, the `getvar` result is `"zebra"` .


1283


SGD Network Commands