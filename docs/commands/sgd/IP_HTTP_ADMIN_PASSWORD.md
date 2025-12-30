# ip.http.admin_password



This command sets the password to be used for authentication on the print server web pages.


**NOTE:** For printers purchased in the EMEA region after August 1, 2025, a password must be
defined before you can use many of the printer's web pages.


**NOTE:** With Protected Mode enabled, many of the printer's web pages do not work.


**Setvar**


To set print server web page password:

```
       ! U1 setvar "ip.http.admin_password" "value"

```

**Values**


A string with a maximum of 25 characters.


**Default**


              - **For printers purchased in the EMEA region after August 1, 2025:** The default value is empty
and must be set before it can be used on the printer.

              - **For all other printers:** `1234`


**Getvar**


To respond with the print server web page password:

```
       ! U1 getvar "ip.http.admin_password"

```

**Example**

This `setvar` example shows the value set to `"P@ssword101"` .

```
       ! U1 setvar "ip.http.admin_password" "P@ssword101"

```

1256


SGD Network Commands