# ^WE




ZPL Network Commands


Use this command to command enable Wired Equivalent Privacy (WEP) mode and set WEP values. WEP is
a security protocol for wireless local area networks (WLANs).


**Set WEP Mode**


**NOTE:**

- The `^WE` command is provided only for backward-compatibility with printers using firmware
prior to V50.15.x, V53.15.x, or X60.15.x. For these firmware versions and later, use **^WX on**
**page 425** to set the security type and related parameters.


- This command does not apply to printers running Link-OS v6 or later versions.


Be careful to include the exact number of commas required for this command when setting encryption
keys (parameters `e` through `h` ). A missing or extra comma will cause the keys to be stored in the wrong
slots and can prevent the printer from joining the wireless network.

**Format:** `^WEa,b,c,d,e,f,g,h`





|Parameters|Details|
|---|---|
|`a` = encryption mode|**Values:**<br>`OFF`<br>`40 =`40-bit encryption<br>`128 =`128-bit encryption<br>**Default:**`OFF`|
|`b` = encryption index|Tells the printer which encryption key to use.<br>**Values:**<br>`1 =`Key 1<br>`2 =`Key 2<br>`3 =`Key 3<br>`4 =`Key 4<br>**Default:**`1`|
|`c` = authentication type|**Values:**O (Open System), S (Shared Key)<br>`O =`Open System<br>`S =`Shared Key<br>**Default:**`O`<br>**NOTE:** If you enable Shared Key authentication with Encryption<br>Mode set to`OFF`, this value resets to`O` (Open).|
|`d` = encryption key<br>storage|**Values:**H (Hex key storage), S (string key storage)<br>`H =`Hex key storage<br>`S =`String key storage<br>**Default:**`H`|


393


ZPL Network Commands






|Parameters|Details|
|---|---|
|e, f, g, h = encryption keys<br>1 through 4|**Values:**The actual value for the encryption key<br>The encryption mode affects what can be entered for the encryption keys:<br>•<br>For 40-bit, encryption keys can be set to any 5 hex pairs or any 10<br>alphanumeric characters.<br>•<br>For 128-bit, encryption keys can be set to any 13 hex pairs or any 26<br>alphanumeric characters.<br>**NOTE:** When using hex storage, do not add a leading 0x on the<br>WEP key.|



**Example:** This example sets encryption to 40-bit, activates encryption key 1, and sets encryption key 1 to
the string `12345` .

```
^WE40,,,,12345

```

In this example, the Encryption Index, Authentication Type, and Encryption Key Storage parameters are left
blank with commas as placeholders for the fields. The printer uses the default values for these parameters.


**Example:** This example sets encryption to 128-bit, activates encryption key 2, and sets encryption keys 1
and 2 to hex values.

```
^WE128,2,,H,12345678901234567890123456,98765432109876543210987654

```

The value for encryption key 1 is stored and can be activated in the future by the following command:

```
^WE128,1

```

**Example:** This example sets encryption to 128-bit, activates encryption key 4, and sets encryption key 4 to
a hex value.

```
^WE128,4,,H,,,,98765432109876543210987654

```

Values are not required for encryption keys 1 through 3 when setting encryption key 4. In this


example, commas are used as placeholders for the fields for encryption keys 1 through 3.


Any previously stored values for these encryption keys do not change.


**NOTE:** **important:** Make sure that you include the exact number of commas required to get to the
slot for encryption key 4 (parameter h).


394


ZPL Network Commands