# ip.ipp.mode



This command sets or retrieves the Internet Printing Protocol (IPP) setting. The printer can be set for either
IPP/IPPS or IPPS (Internet Printing Protocol Secure)-only operation.


**Setvar**


To set the IPP mode:

```
       ! U1 setvar "ip.ipp.mode" "value"

```

**Values**

              - `"ipp/ipps"` enables IPP/IPPS

              - `"ipps"` enables IPPS-only

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"ipps"`

              - **For all other printers:** `"ipp/ipps"`


**Getvar**


To retrieve the current setting of the IPP mode:

```
       ! U1 getvar "ip.ipp.mode"

```

**Example**

This `setvar` example shows the value set to `"ipps"` .

```
       ! U1 setvar "ip.ipp.mode" "ipps"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"ipps"` .


1265


SGD Network Commands