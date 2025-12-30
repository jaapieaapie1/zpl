# wlan.pmf




SGD Network Commands


This command allows the user to configure the 802.11 Protected Management Frame settings. This is only
applicable for printers having the 802.11ac radio feature.


**Setvar**


To configure the protected management feature setting:

```
! U1 setvar "wlan.pmf" "value"

```

**Values**

`"disabled"` means Protected Management Frame is disabled

`"enabled"` means Protected Management Frame is enabled

`"required"` means Printer must support PMF.

**Default**
```
   "enabled"

```

**Getvar**


To return the current setting value:

```
! U1 getvar "wlan.pmf"

```

1464


SGD Network Commands