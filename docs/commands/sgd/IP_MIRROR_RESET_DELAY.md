# ip.mirror.reset_delay



This command specifies the number of seconds between when the printer receives the last byte of the last
file from the `/commands` directory and when the printer resets during a mirror event.


**Setvar**

To set the number of seconds between when the printer receives the last byte of the last file from the `/`
`commands` directory and when the printer resets during a mirror event:

```
       ! U1 setvar "ip.mirror.reset_delay" "value"

```

**Values**

`"0"` to `"900"` (seconds)

**Default**
```
          "5"

```

**NOTE:** The default setting for the `ip.mirror.reset_delay` command is 5 seconds;
in some cases it may be necessary to use a longer delay to allow for full processing of
longer or more complex files.


**Getvar**


To retrieve the number of seconds between when the printer receives the last byte of the last file from
the /commands directory and when the printer resets during a mirror event:

```
       ! U1 getvar "ip.mirror.reset_delay"

```

**Example**

This `setvar` example shows the value set to `"10"` .

```
       ! U1 setvar "ip.mirror.reset_delay" "10"

```

When the `setvar` value is set to `"10"`, the `getvar` result is `"10".`


1284


SGD Network Commands