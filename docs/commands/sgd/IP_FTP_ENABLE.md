# ip.ftp.enable



This printer setting refers to the FTP protocol setting. This command tells the printer to turn FTP on or off.


**Setvar**


To turn FTP on or off:

```
       ! U1 setvar "ip.ftp.enable" "value"

```

**Values**

`"off"` disables FTP

`"on"` enables FTP

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To respond with the FTP status:

```
       ! U1 getvar "ip.ftp.enable"

```

**Example**

This `setvar` example shows the FTP status set to `"on"` .

```
       ! U1 setvar "ip.ftp.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is that the FTP status is `"on"` .


1251


SGD Network Commands