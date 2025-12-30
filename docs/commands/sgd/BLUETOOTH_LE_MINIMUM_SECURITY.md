# bluetooth.le.minimum_security



Determines the minimum device-level security settings the printer will use to connect for Bluetooth LowEnergy (LE) connections.


**Setvar**


To set the device-level security settings the printer will use to connect for Bluetooth Low-Energy (LE)
connections:

```
       ! U1 setvar "bluetooth.le.minimum_security" "value"

```

**Values**

              - `"none"` security is not required unless the particular Bluetooth LE service or characteristic
being accessed requires security. For all other choices, all services will require pairing and apply
some form of security.

              - `"unauth_key_signing"` (un)authenticated pairing and signing keys are required

              - `"auth_key_signing"` authenticated pairing and signing keys are required.

              - `"unauth_key_encrypt"` (un)authenticated pairing and encryption are required.

              - `"auth_key_encrypt"` authenticated pairing and encryption are required.

**Default**

`"none"` if the printer has a Bluetooth LE radio. If the printer does not have a Bluetooth LE radio
installed, there is no default value.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "bluetooth.le.minimum_security"

```

If the printer does not have a Bluetooth LE radio installed, the printer will return an empty string.


1104


SGD Network Commands