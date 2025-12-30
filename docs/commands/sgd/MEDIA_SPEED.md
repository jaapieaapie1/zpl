# media.speed



This command specifies media print speed in inches per second (ips).


**Setvar**


This command instructs the printer to set the media print speed.

```
       ! U1 setvar "media.speed" "value"

```

**Values**
`"2"` to `"12"` ips

              - `"up"` increments the printer speed by one unit

              - `"down" =` decrements the speed by one unit

**Default**
```
          "2"

```

**Getvar**


This command retrieves the currently set media print speed.

```
       ! U1 getvar "media.speed"

```

**Example**

This `setvar` example shows the value set to `"2"` .

```
       ! U1 setvar "media.speed" "2"

```

When the `setvar` value is set to `"2"`, the `getvar` result is `"2"` .

This `setvar` example shows the value set to `"up"` .

```
       ! U1 setvar "media.speed" "up"

```

If the current print speed is 2 and the `setvar` value is set to `"up"`, the `getvar` result is `"3"` .

This `setvar` example shows the value set to `"down"` .

```
       ! U1 setvar "media.speed" "down"

```

If the current print speed is 2 and the `setvar` value is set to `"down"`, the `getvar` result is `"1"` .


877


SGD Printer Commands