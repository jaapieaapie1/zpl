# wlan.roam.neighbor_assist



This command supports 802.11k roaming. When enabled and connected to a network, the printer will query
the neighbor list from the Access Point (AP) and use the neighbor AP channel list to reduce the number of
channels needed to scan during roaming attempts.


**Setvar**

To enable `wlan.roam.neighbor_assist` :

```
       ! U1 setvar "wlan.roam.neighbor_assist" "on"

```

**Values**

              - `on` enables the neighbor query.

              - `off` disables the neighbor query.

**Default**
```
          off

```

**Getvar**

To return the value of `wlan.roam.neighbor_assist` :

```
       ! U1 getvar "wlan.roam.neighbor_assist " "on"

```

**Values**

              - `on` the neighbor query is enabled.

              - `off` the neighbor query is disabled.


1474


SGD Network Commands