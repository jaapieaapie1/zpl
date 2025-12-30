# device.frontpanel.key_press



This command instructs the printer to press a button on the front panel.


**Setvar**


To instruct the printer to press a button on the front panel:

```
       ! U1 setvar "device.frontpanel.key_press"

```

**Values**


The values vary per printer, as follows:


**ZM400, Z4M/Z6M, and RZ400/RZ600**

              - `"A"` Pause

              - `"B"` Feed

              - `"C"` Cancel

              - `"D"` Setup/Exit

              - `"E"` Minus

              - `"F"` Select

              - `"G"` Plus

**XiIIIplus**

              - `"A"` Pause

              - `"B"` Feed

              - `"C"` Cancel

              - `"D"` Setup/Exit

              - `"E"` Previous

              - `"F"` Next/Save

              - `"G"` Minus

              - `"H"` Plus

              - `"I"` Calibrate

**S4M**

              - `"A"` Pause

              - `"B"` Feed

              - `"C"` Up Arrow

              - `"D"` Cancel

              - `"E"` Menu

              - `"F"` Enter


698


SGD Printer Commands


**Xi4, RXi4**

    - `"A"` Pause

    - `"B"` Feed

    - `"C"` Cancel

    - `"D"` Setup/Exit

    - `"E"` Previous

    - `"F"` Next/Save

    - `"G"` Minus

    - `"H"` Plus

    - `"I"` Calibrate


**Example**

This `setvar` example shows the value set to `"A"` .

```
! U1 setvar "device.frontpanel.key_press" "A"

```

699


SGD Printer Commands