# alerts.tracked_settings.clear_log



This command clears the `alerts.tracked_settings.log` . Setting this command to any value,
including an empty string, will clear the `tracked_sgds` log entries.


**Setvar**

To clear the `tracked_sgds` log entries:

```
       ! U1 setvar "alerts.tracked_settings.clear_log" "value"

```

**Values**


Any string value, including an empty string.


**Default**


NA


**Do**

To clear the `tracked_sgds` log entries:

```
       ! U1 do "alerts.tracked_settings.clear_log" "value"

```

**Values**


Any string value, including an empty string.


**Default**


NA


**Example**


This example clears the log entries with an empty string value.

```
       ! U1 setvar "alerts.tracked_settings.clear_log" ""

```

625


SGD Printer Commands