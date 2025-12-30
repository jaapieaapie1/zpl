# media.darkness_mode



This command instructs the printer to set the darkness mode.


**Setvar**


To set the darkness mode:

```
       ! U1 setvar "media.darkness_mode" "value"

```

**Values**

              - `"cartridge"` indicates cartridge mode (no changes allowed

              - `"user"` indicates user mode (Darkness is set by the user, and the cartridge value is ignored.
This value is used for all cartridges inserted in the printer).

              - `"relative"` indicates relative mode (the specified darkness value is added to the cartridge
default value)


**Default**
```
          "cartridge"
```

**Example**

This `setvar` example shows the darkness mode set to `"cartridge"` .

```
          ! U1 setvar "media.darkness_mode" "cartridge"

```

857


SGD Printer Commands