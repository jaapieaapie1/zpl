# print.legacy_compatibility



This command turns off or on the legacy compatibility print quality.


**Setvar**


To enable or disable the legacy compatibility print quality:

```
       ! U1 setvar "print.legacy_compatibility" "value"

```

**Values**

              - `"on"` uses legacy QLn print quality tables. Applicable to ZQ610 and ZQ620 printers only. Not
supported on ZQ630.

              - `"off"` uses ZQ6 print quality tables.

**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "print.legacy_compatibility"

```

961