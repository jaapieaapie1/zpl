# ip.dhcp.cid_enable



This command determines if DHCP (option 61) is turned on or off.


**Setvar**


To set the status of the client identifier:

```
       ! U1 setvar "ip.dhcp.cid_enable" "value"

```

**Values**

              - `"off"` client identifier is turned off

              - `"on"` client identifier is turned on

**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.dhcp.cid_enable"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "ip.dhcp.cid_enable" "off"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"off"` .


1224


SGD Network Commands