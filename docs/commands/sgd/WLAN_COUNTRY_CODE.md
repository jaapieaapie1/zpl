# wlan.country_code



This command defines the regulatory country for which the radio is currently configured.


**Setvar**


To set the country code for which the radio is to be configured:

```
       ! U1 setvar "wlan.country_code" "value"

```

**IMPORTANT:** The list of country codes is specific to each printer and depends on the printer
model and its wireless radio configuration. The list is subject to change, addition, or deletion with
any firmware update, at any time, without notice.

To determine the country codes available on your printer, issue the `! U1 getvar "wlan"` command to
return all commands related to WLAN settings. Locate the `wlan.country.code` command in the results
and view the country codes available for your printer.


**Getvar**


To retrieve the country code for which the radio is currently configured:

```
       ! U1 getvar "wlan.country_code"

```

**Example**

In this example, the `setvar` sets the country code to USA/Canada.

```
       ! U1 setvar "wlan.country_code" "usa/canada"

```

1394


SGD Network Commands