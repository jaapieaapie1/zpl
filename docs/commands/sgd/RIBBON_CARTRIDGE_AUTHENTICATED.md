# ribbon.cartridge.authenticated



This command returns the printer cartridge authentication status. The setting gets updated every time an
authentication occurs (power up, head close, or any other time).


If a ribbon cartridge is not installed, then the printer returns "not installed". If the ribbon cartridge option is
not present, then the command returns an empty string.


**Getvar**


To return the printer cartridge authentication status:

```
       ! U1 getvar "ribbon.cartridge.authenticated"

```

**Result**

`"yes"` The cartridge installed is authenticated.

`"no"` The cartridge installed is not authenticated.

`"not installed"` The printer supports ribbon cartridge, but it is not installed (initial
condition prior to authentication).

`"" (empty string)` The printer does not support ribbon cartridge.


**Example**

In this example, the `getvar` returns that the cartridge is authenticated.

```
       ! U1 getvar "ribbon.cartridge.authenticated" "yes"

```

969


SGD Printer Commands