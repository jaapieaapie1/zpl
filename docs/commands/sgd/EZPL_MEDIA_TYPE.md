# ezpl.media_type



This command specifies the media type being used. This command is similar to the `^MN` ZPL command.


**Setvar**


To set the media type used in the printer:

```
       ! U1 setvar "ezpl.media_type" "value"

```

**Values**

              - `"continuous"`

              - `"gap/notch"`

              - `"mark"`

**Default**
```
          "gap/notch"

```

**Getvar**


To return the current media type setting:

```
       ! U1 getvar "ezpl.media_type"

```

**Example**

This `setvar` example sets the media type to `"continuous"` .

```
       ! U1 setvar "ezpl.media_type" "continuous"

```

810




SGD Printer Commands