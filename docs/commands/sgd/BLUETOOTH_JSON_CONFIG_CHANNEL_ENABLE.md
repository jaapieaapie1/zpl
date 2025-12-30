# bluetooth.json_config_channel_enable



Enables or disables the Bluetooth JSON configuration channel. Changes to this command setting only take
effect when the printer is reset or power cycled.


**NOTE:** The JSON config channel will stop being advertised when there is no connection to the
JSON channel and there is no connection on the main SPP channel.


**IMPORTANT:** All Bluetooth devices must first make an SPP connection before being able to
make the JSON channel connection to a printer.


**Setvar**


To enables or disables the Bluetooth JSON configuration channel:

```
       ! U1 setvar "bluetooth.json_config_channel_enable" "value"

```

**Values**

              - `"on"` means the channel is advertised and available for use only when the main serial port
protocol (SPP) channel is connected.

              - `"off"` means the channel is not advertised or available for use.

**Default**
```
          "on"

```

**Example**

```
       ! U1 setvar "bluetooth.json_config_channel_enable" "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "bluetooth.json_config_channel_enable"

```

1100




SGD Network Commands