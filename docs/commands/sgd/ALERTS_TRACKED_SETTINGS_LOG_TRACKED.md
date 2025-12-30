# alerts.tracked_settings.log_tracked



This command creates a comma-delimited list of settings for which sets should be logged.


**Setvar**


To set the list of settings for which sets should be logged:

```
       ! U1 setvar "alerts.tracked_settings.log_tracked"
       "settings.name1,settings.name2..."

```

**Values**


Settings with commas between names.


**Default**
```
          ""

```

**Getvar**


To return a comma-delimited lists of settings being logged:

```
       ! U1 getvar "alerts.tracked_settings.log_tracked"

```

**Do**


To set the list of settings for which sets should be logged:

```
       ! U1 do "alerts.tracked_settings.log_tracked"
       "settings.name1,settings.name2..."

```

**Values**


Settings with commas between names.


**Default**
```
          ""

```

626


SGD Printer Commands