# wlan.11n.aggregation



This command enable or disables Aggregation MAC Service Data Unit (A-MSDU) in 802.11n radio
communications.


**Setvar**


To enable or disable A-MSDU in 802.11n radio communications:

```
       ! U1 setvar "wlan.11n.aggregation" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To return the current setting for A-MSDU:

```
       ! U1 getvar "wlan.11n.aggregation"

```

**Example**

```
       ! U1 setvar "wlan.11n.aggregation" "on"

```

1364


SGD Network Commands