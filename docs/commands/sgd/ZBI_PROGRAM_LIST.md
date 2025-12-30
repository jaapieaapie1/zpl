# zbi.program_list



This SGD is intended to be used on WML pages. It purpose is to support displaying the name of each ZBI
program file contained on the E: drive.


**IMPORTANT:** A specific sequence required to get an accurate listing of files. See the example
below.


**Setvar**


To display the progam list:

```
       ! U1 setvar "zbi.program_list" "value"

```

**Values**

              - `"fill"` initializes the device (first step)

              - `"up"` gets the previous file in the list

              - `"down"` gets the next file in the list

              - `"execute"` executes the currently specified zbi file (as determined by the `getvar` command)

**Default**

              - `"none"`


**Example**


A specific sequence of commands is required to get an accurate listing of files.


Initialize the device by issuing:

```
       ! U1 setvar "zbi.program_list" "fill"

```

To get the first file, issue:

```
       ! U1 getvar "zbi.program_list"

```

This will return the current filename in the list of zbi files present on the E: drive. If the response returned is
`"none"` you have reached the end of the list.

To get the next filename in the list, issue:

```
       ! U1 setvar "zbi.program_list" "up"

```

To get the previous filename in the list, issue:

```
       ! U1 setvar "zbi.program_list" "down"

```

To execute the current zbi filename, i.e. the one returned by a getvar, issue:

```
       ! U1 setvar "zbi.program_list" "execute"

```

1060




SGD Printer Commands


**Getvar**


To return the current setting value:

```
! U1 getvar "zbi.program_list"

```

1061


SGD Printer Commands