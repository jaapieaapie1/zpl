# wlan.auth_type



This parameter selects the authentication service to be used between the printer and the Access Point.
Open System and Shared Key are the two types of authentication services.


**Setvar**


To instruct the printer to set the authentication type to the specified value:

```
       ! U1 setvar "wlan.auth_type" "value"

```

**Values**

              - `"open"`

              - `"shared"`

**Default**
```
          "open"

```

**Getvar**


To instruct the printer to retrieve the current authentication type:

```
       ! U1 getvar "wlan.auth_type"

```

**Example**


This example instructs the printer to set the authentication type to Shared Key.

```
       ! U1 setvar "wlan.auth_type" "shared"

```

The authentication type will ble set to Shared Key after power cycle.


1389


SGD Network Commands