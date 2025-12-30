# alerts.http.logging.clear



This command clears the weblink alerts log entries. It does not disable logging. Setting this command to
any value, including an empty string, will clear the weblink log entries.


**Setvar**


To clear the weblink alerts log entries:

```
       ! U1 setvar "alerts.http.logging.clear" "value"

```

**Value**


Any string value, including an empty string.


**Default**


NA


**Do**


To clear the weblink alerts log entries:

```
       ! U1 do "alerts.http.logging.clear" "value"

```

**Values**


Any string value, including an empty string.


**Default**


NA


**Example**


This example clears the log entries with an empty string value.

```
       ! U1 setvar "alerts.http.logging.clear" ""

```

618


SGD Printer Commands