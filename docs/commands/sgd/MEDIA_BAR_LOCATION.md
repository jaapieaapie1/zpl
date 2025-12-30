# media.bar_location



Allows the user to configure the printer to look for a black mark bar on the front or back of the media.


**NOTE:** This command works only with printers that have a front media sensor.


**Setvar**


To configure the printer’s black bar location:

```
       ! U1 setvar "media.bar_location" "value"

```

**Values**

              - `"front"` uses media with bars on the front.

              - `"back"` uses media with bars on the back

Options available by printer:

              - iMZ220, iMZ320, QLn220, QLn320, QLn420, ZR318, ZR328 use `"front"`

              - ZQ310, ZQ320, ZQ510, ZQ520 use `"front", "back"`

              - All other printers use `"back"`

**Default**

              - iMZ220, iMZ320, QLn220, QLn320, QLn420, ZR318, ZR328 use `"front"`

              - ZQ310, ZQ320 with optional back bar sensor use `"back"`

              - ZQ310, ZQ320 with no back bar sensor use `"front"`

              - All other printers use `"back"`


**Getvar**


To return the current setting value:

```
       ! U1 getvar "media.bar_location"

```

847


SGD Printer Commands