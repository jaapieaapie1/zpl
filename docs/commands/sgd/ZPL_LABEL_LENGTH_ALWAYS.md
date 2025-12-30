# zpl.label_length_always



This command allows the label length defined by `^LL` to apply when the media is Gap or Black Mark. This
command is persistent across a power cycle.


See also ^LL on page 294.


**Setvar**


To enable or disable the label length parameter:

```
       ! U1 setvar "zpl.label_length_always" "value"

```

**Values**

              - `"yes"` applies the label length to all media.

              - `"no"` causes the label length to apply only to continuous media.


**Getvar**

To determine the setting for `zpl.label_length_always` :

```
       ! U1 getvar "zpl.label_length_always"

```

**Values**

              - `"yes"` indicates that the label length applies to all media.

              - `"no"` indicates that the label length applies only to continuous media.

**Default**
```
          "no"

```

1075


SGD Printer Commands