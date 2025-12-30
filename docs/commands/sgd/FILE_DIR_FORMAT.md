# file.dir_format



This command controls the output format of the `file.dir` Set/Get/Do command.


**Setvar**


To set the output format:

```
       ! U1 setvar "file.dir_format" "value"

```

**Values**

              - `"cpcl"`

              - `"zpl"`


**Getvar**


To retrieves the current setting for the output format:

```
       ! U1 getvar "file.dir_format"

```

**Result**

              - `"cpcl"`

              - `"zpl"`


**Do**


To set the output format:

```
       ! U1 do "file.dir_format" "value"

```

**Values**

              - `"cpcl"`

              - `"zpl"`


**Example**

This `do` example sets the directory format to `CPCL` .

```
       ! U1 do "file.dir_format" "cpcl"

```

825