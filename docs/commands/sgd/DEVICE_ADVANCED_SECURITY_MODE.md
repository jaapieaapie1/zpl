# device.advanced_security_mode



This command reports if Advanced Security Mode (ASM) is enabled or disabled.


[For more information about Advanced Security Mode, refer to the Zebra Link-OS PrintSecure Printer](https://www.zebra.com/content/dam/support-dam/en/documentation/unrestricted/guide/software/printsecure-administration-guide-en.pdf)
[Administration Guide.](https://www.zebra.com/content/dam/support-dam/en/documentation/unrestricted/guide/software/printsecure-administration-guide-en.pdf)


**Getvar**


To determine whether ASM is enabled:

```
       ! U1 getvar "device.advanced_security_mode"

```

**Values**

              - `"advanced"` ASM is enabled.

              - `"none"` ASM is disabled.


662


SGD Printer Commands