# alerts.tracked_sgds.zbi_notified



This command provides a comma-delimited list of settings for which ZBI should be notified when the value
is set.


**Setvar**


To set the list of the settings for which ZBI will be notified when the setting is set:

```
       ! U1 setvar "alerts.tracked_settings.zbi_notified"
       "settings.name1,settings.name2,etc."

```

**Values**


A comma delimited list of settings names.


**Default**
```
          ""

```

**Getvar**


To retrieve the list of the settings for which ZBI will be notified when the value is set:

```
       U1 getvar "alerts.tracked_settings.zbi_notified"

```

**Do**


To set the list of the settings for which ZBI will be notified when the setting is set:

```
       ! U1 setvar "alerts.tracked_settings.zbi_notified"
       "settings.name1,settings.name2,etc."

```

**Values**


A comma delimited list of settings names.


**Default**
```
          ""

```

630




SGD Printer Commands