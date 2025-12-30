# device.frontpanel.line2



This command overrides the content that is shown on the second line of the front panel when the printer is
showing the idle display. Use of the `getvar` function is dependent on using the `setvar` function.

For example, to have the second line of the idle display show HELLO, you must first send a `setvar`
command; then a `getvar` command can be sent to retrieve the value HELLO.

For details on the supported character set, see ZBI Character Set on page 1605.


**Setvar**


To instruct the printer to set the content that shows on line two of the front panel:

```
       ! U1 setvar "device.frontpanel.line2" "value"

```

**Values**


The maximum amount of alphanumeric ASCII characters available for line two on the printer’s front
panel.


**Default**
```
          ""

```

**Getvar**


To retrieve the content that shows on line two of the front panel:

```
       ! U1 getvar "device.frontpanel.line2"

```

**Example**

This `setvar` example shows the value set to `"sample line 2"` .

```
       ! U1 setvar "device.frontpanel.line2" "sample line 2"

```

When the `setvar` value is set to `"sample line 2"`, the `getvar` result is `"sample line 2"` .


701


SGD Printer Commands