# comm.pnp_option



The `comm.pnp_option` command configures the RS-232 Serial Port Plug and Play setting on the printer.


**Setvar**

To instruct the printer to change the `comm.pnp_option` setting:

```
       ! U1 setvar "comm.pnp_option" "value"

```

**Values**

              - `"off"` sets the printer to not attempt Plug and Play via serial at startup

              - `"on"` sets the printer to attempt Plug and Play via serial at startup

**Default**
```
          "off"

```

**Getvar**

To return the current setting for the `comm.pnp_option` setting:

```
       ! U1 getvar "comm.pnp_option"

```

**Example**

In this example, the `setvar` sets the serial port communications state to `"on"` .

```
       ! U1 setvar "comm.pnp_option" "on"

```

When the setvar value is set to `"on"`,the `getvarresult` is `"on"` .


**NOTE:** Not all Operating Systems and computer hardware support Plug and Play over
RS-232
Serial port connections.


653


SGD Printer Commands