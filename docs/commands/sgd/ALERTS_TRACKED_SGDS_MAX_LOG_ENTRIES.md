# alerts.tracked_sgds.max_log_entries



This command sets the maximum number of entries to be shown in `alerts.tracked_settings.log` .


**Setvar**


To set the maximum number of alert log entires that will be stored:

```
       ! U1 setvar "alerts.tracked_sgds.max_log_entries" "value"

```

**Values**

`"0"` to `"10000"`

**Default**
```
          "100"
```

Setting the value to `"0"` disables logging.


**Getvar**


To return the setting for the maximum number of alert log entires that will be stored:

```
       ! U1 getvar "alerts.tracked_sgds.max_log_entries"

```

**Do**


To set the maximum number of alert log entires that will be stored:

```
       ! U1 setvar "alerts.tracked_sgds.max_log_entries" "value"

```

**Values**

`"0"` to " `10000"`

**Default**
```
          "100"
```

Setting the value to `"0"` disables logging.


**Example**

This example sets the maximum log entries to `"50"` .

```
       ! U1 setvar "alerts.tracked_sgds.max_log_entries" "50"

```

629


SGD Printer Commands