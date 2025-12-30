# device.company_contact



This command sets the company contact information, which can be accessed from the `server/`
`sysinfo.htm` webpage.


**Setvar**


To set the company contact information:

```
       ! U1 setvar "device.company_contact" "value"

```

**Values**


A string up to 128 characters in length.


**Result**
```
          ""

```

**Getvar**


To return the current company contact information:

```
       ! U1 getvar "device.company_contact"

```

**Example**

This `setvar` example shows the value set to `"Zebra"` .

```
       ! U1 setvar "device.company_contact" "Zebra"

```

680




SGD Printer Commands