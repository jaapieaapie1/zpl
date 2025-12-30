# ip.dhcp.auto_provision_enable



This command prepares the printer to receive Weblink settings from the DHCP server. These settings are
used to allow the printer to connect to on-premise or web-based servers.

If DHCP receives option 43 in the format of `"10642 <auto provision code> < auto provision`
`settings>”` and the Client is not already configured, the client will use the configuration to connect to
the server.


Using this feature requires configuring your DHCP server to send option 43 information during address
assignment. The feature allows the printer to obtain the settings used to control Cloud Connect weblink
connections or Mirror events as part of receiving a DHCP assigned IP address. This requires that DHCP
option 60 is not empty and that `ip.dhcp.auto_provision_enable` is set to `"on"` .

The package of Cloud Connect/weblink information sent from the DHCP server in the Option 43 response
can include the:


          - Server address


          - Authentication server name


          - User name and password for proxy logins


The package of Mirror information sent from the DHCP server in the Option 43 packet can include the:


          - Server address


          - Mirror path


          - Mirror feedback path


          - Mirror appl path


          - Mirror mode


**Setvar**


To enable or disable the DHCP auto provision feature of the printer:

```
       ! U1 setvar "ip.dhcp.auto_provision_enable" "value"

```

**Values**

              - `"on"` enabled

              - `"off”` disabled

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To retrieve the current value of on or off for the DHCP auto provision feature:

```
       ! U1 getvar "ip.dhcp.auto_provision_enable"

```

1221


SGD Network Commands