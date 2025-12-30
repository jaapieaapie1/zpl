# media.cut_now



This command instructs the printer cycle the media cutter. If the printer is in Print Mode Kiosk
(media.printmode “K”) then the cutter will execute a cut based on the value of media.present.cut_amount

         - either a normal cut or a partial cut. If the printer is not in Print Mode Kiosk (media.printmode “K”), this
command does nothing. .


See media.present.cut_amount on page 866.


**Setvar**


To instruct the printer to cycle the media cutter:

```
       ! U1 setvar "media.cut_now" ""

```

**NOTE:** See media.present.cut_amount on page 866.


**Do**


To instruct the printer to cycle the media cutter:

```
       ! U1 do "media.cut_now" ""

```

856


SGD Printer Commands