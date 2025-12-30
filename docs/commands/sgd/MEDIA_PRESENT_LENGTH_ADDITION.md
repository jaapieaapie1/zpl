# media.present.length_addition



This printer setting adds an additional amount to how far the paper is ejected during a present cycle. A
standard amount of 50mm is always added to clear the kiosk wall. This amount is added to that 50mm. The
total amount of media ejected this command is executed, then, is 50mm + media.present.length_addition +
media.present.eject.


**Setvar**


This command instructs the printer to change the media present length addition.

```
       ! U1 setvar "media.present.length_addition" "value"

```

**Values**
`"0"` to `"255"` specifies the additional mm of media to eject


**Getvar**


This command instructs the printer to respond with the currently set media present length addition.

```
       ! U1 getvar "media.present.length_addition"

```

868


SGD Printer Commands