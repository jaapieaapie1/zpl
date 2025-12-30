# power.wake.radio



This command is used to enable or disable the power wake feature on printers that are radio (WLAN,
BT Classic, and BTLE) enabled. The radio must be enabled to support waking on that interface. i.e.
wlan.enable must be set to `"yes"` to support the wake feature on WLAN.


**Setvar**


To enable or disable the power wake setting:

```
       ! U1 setvar "power.wake.radio" "values"

```

**Values**

`"on"` `"off"` ZQ6

`"on"` `"off"` ZQ3

`"on"` ZQ3 with BT Classic/BTLE radio installed (BT Only - UART)

`"off"` ZQ5 with WLAN/BT Classic radio installed

`"on"` ZQ5 with BT Classic/BTLE radio installed (BT Only - UART)

`"on"` `"off"` ZD4xx, ZD6xx with WLAN/BT Classic/BTLE installed

`"off"` ZD4xx, ZD6xx with BTLE only radio or no radio installed


**Default**
```
          "on"

```

**Getvar**


To return the current setting:

```
       ! U1 getvar "power.wake.radio"

```

953


SGD Printer Commands