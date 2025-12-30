# alerts.tracked_settings.max_log_entries



Sets or retrieves the maximum number of entries to be shown in the `alerts.tracked_settings.log` .


**Setvar**


To set the maximum number of entries:

```
       ! U1 setvar "alerts.tracked_settings.max_log_entries" "value"
       ! U1 do "alerts.tracked_settings.max_log_entries" "value"

```

**Values**

`"0"` to `"10000"`

**Default**
```
          "100"

```

**Getvar**


To retrieve the maximum number of entries :

```
       ! U1 getvar "alerts.tracked_settings.max_log_entries"

```

627


SGD Printer Commands