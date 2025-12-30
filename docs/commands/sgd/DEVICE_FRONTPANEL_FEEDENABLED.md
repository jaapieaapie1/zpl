# device.frontpanel.feedenabled



This command can be used to enable or disable the **FEED** key or any other key on the printer.


**Setvar**

To instruct the printer to change the `front_panel.feedenable` d setting:

```
       ! U1 setvar "device.frontpanel.feedenabled"

```

**Values**

              - `"yes"` Front Panel keys are enabled

              - `"no"` Front Panel keys are disabled

**Default**

              - `"no"` for GX420s printers

              - `"yes"` all supported printers except GX420s

**Power On Default**

              - `"no"` for GX420s printers

              - `"yes"` all supported printers except GX420s


**Getvar**

To retrieve the current setting for the `front_panel.feedenable` command:

```
       ! U1 getvar "device.frontpanel.feedenabled"

```

**Example**

In this example, the `setvar` sets the value to `"no"` .

```
       ! U1 setvar "device.frontpanel.feedenabled" "no"

```

**NOTE:**


          - On GX420 printers with an LCD display, there is a **SCROLL** and **SELECT** key in addition to the
**FEED** key. Both the **SCROLL** and **SELECT** keys are enabled or disabled when the **FEED** key is
enabled or disabled using this command.

          - On power up, for model GX420s printer, the command value is set to `"no"` . For all other
printers, on power up, the command value is set to `"yes"` .


697


SGD Printer Commands