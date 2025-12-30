# media.tof




SGD Printer Commands


Sets the offset of the black mark or label gap to the point of separation between documents.


**Related ZPL Command**
```
^MN

```

**Setvar**


To set the black mark offset value:

```
! U1 setvar "media.tof" "value"

```

**Values**

    - `0` is the point of separation, such as the perforation or cut point

    - `-84` to `64` for the MZ printer

    - `-400` to `400` for all other Mobile printers

    - `-400` is the lower limit for all other printers

For all other printers, the upper limit depends on the print mode (Thermal Transfer or Direct
Thermal), the printer model, the print head density, and the current value of Label Top.


**Default**
```
   0
```

**Example**

```
   ! U1 setvar "media.tof" "-50"

```

**Getvar**


To return the current setting value:

```
! U1 getvar "media.tof"

```

**Result**
```
   "-50"

```

**IMPORTANT:** The coordinate system for `media.tof` is reversed from ZPL; therefore
`media.tof` is the negative of the b value of `^MNM,b` . If `^MNM,b` sets the offset value to
`"50"`, then the `media.tofgetvar` value returned will be `"-50"` .


**Example**

```
^XA^MNM,50^XZ
! U1 getvar "media.tof"

```

**Result**
```
   "-50"

```

879


**Supported Devices**


- iMZ220, iMZ320


- QLn220, QLn320, QLn420


- ZD220, ZD230


- ZD410, ZD420


- ZD500, ZD510


- ZD511


- ZD620


- ZD888


- ZQ120, ZQ220


- ZQ310, ZQ320


- ZQ510, ZQ520


- ZQ610, ZQ620



SGD Printer Commands


880




SGD Printer Commands