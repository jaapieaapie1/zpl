# media.present.eject



This command instructs the printer to eject the document through the presenter module. The value is the
amount to eject, in mm. The value of `media.present.length_addition` gets added to the value to
determine the total length of media ejected.


**Setvar**


This command instructs the printer to eject the document through the presenter module.

```
       ! U1 setvar "media.present.eject" "value"

```

**Values**
`"0"` to " `255"` specifies the amount of media to eject in mm


**NOTE:** See media.present.length_addition on page 868.


**Do**


This command instructs the printer to eject the document through the presenter module.

```
       ! U1 do "media.present.eject" "value"

```

867


SGD Printer Commands