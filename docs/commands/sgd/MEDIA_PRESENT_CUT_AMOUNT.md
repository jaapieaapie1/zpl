# media.present.cut_amount



This printer setting determines the type of cut made by the printer cutter (normal or partial) and, if partial,
the length of the partial cut on each side, in mm.


**Setvar**


To instruct the printer to change the media cut amount:

```
       ! U1 setvar "media.present.cut_amount" "value"

```

**Values**

              - `"0"` indicates a normal cut

              - `"10"` to `"60"` indicates a partial cut where the value indicates the number of mm of media left
uncut


**Getvar**


To respond with the currently set media cut amount:

```
       ! U1 getvar "media.present.cut_amount"

```

866


SGD Printer Commands