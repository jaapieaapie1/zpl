# device.frontpanel.line1



This command overrides the content that is shown on the first line of the front panel when the printer is
showing the idle display. Use of the `getvar` function is dependent on first using the `setvar` function. For
example, to have the first line of the idle display to show HELLO, you must first send a `setvar` command;
then a `getvar` command can be sent to retrieve the value HELLO.


**Setvar**


For details on the supported character set, see ZBI Character Set on page 1605.


To instruct the printer to set the content that is shown on line one of the front panel:

```
       ! U1 setvar "device.frontpanel.line1" "value"

```

**Values**


The maximum amount of alphanumeric ASCII characters available for line 1 on the printer’s front
panel.


**Default**
```
          ""

```

**Getvar**


To retrieves the content that is shown on line one of the front panel:

```
       ! U1 getvar "device.frontpanel.line1"

```

**Example**

This `setvar` example shows the value set to `"sample line 1"` .

```
       ! U1 setvar "device.frontpanel.line1" "sample line 1"

```

When the setvar value is set to `"sample line 1"`, the `getvar` result is `"sample line 1"` .


700




SGD Printer Commands