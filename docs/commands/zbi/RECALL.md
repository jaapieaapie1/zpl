# Recall Program



This program searches for a ZPL format named "FORMAT.ZPL" that is already saved in printer memory. If
the format is found, a number within the format is extracted and shown on the console. The user is then
prompted to enter a new number, which is then substituted into the format.


**Example**


This is an example of Recall.zpl

```
     1 rem ********************************************************
     1 rem Zebra Technologies ZBI Sample Program
     1 rem
     1 rem Professional programming services are available. Please contact
     1 rem ZBI-Experts@zebra.com for more information.
     1 rem
     1 rem This is an example of recalling a ZPL format and extracting data
     1 rem from it.
     1 rem ********************************************************
     1 rem close ports except console, open ZPL port and declare search
     1 rem array
     1 rem ********************************************************
     10 for i = 1 to 9 step 1 ! Close all ports
     20  close #i
     30 next i
     40 let zplport = 2
     50 open #zplport: name "ZPL"
     60 declare string search_zpl$(2)
     70 let search_zpl$(1) = chr$(03)
     80 let search_zpl$(2) = "FORMAT.ZPL"
     1 rem ********************************************************
     1 rem main program; look for format to recall on printer
     1 rem ********************************************************
     90 do
     100  print #zplport: "^XA^HWE:*.ZPL^FS^XZ"
     110    let present = 0
     115  let find$ = ""
     120  do until find$ = chr$(03)
     130    let find$ = searchto$(zplport, search_zpl$)
     140    if find$ = "FORMAT.ZPL" then
     150     let present = 1 ! format is present
     160    end if
     170  loop

     1 rem ********************************************************
     1 rem if format is not found, create a format and set data value to
     1 rem 000
     1 rem ********************************************************
     180  if present = 0 then
     190    print #zplport:"^XA^DFE:FORMAT.ZPL^FS";
     200    print #zplport:"^FX000^FS^XZ"
     210    let counter$ = "000"
     1 rem ********************************************************

```

599


ZBI Commands

```
     1 rem if format is found, extract the data from ^FX field
     1 rem ********************************************************
     220  else
     230    print #zplport:"^XA^HFE:FORMAT.ZPL^FS^XZ"
     240    let stop$ = searchto$(zplport, "^FX")
     250    let counter$ = extract$(zplport, "", "^FS")
     260    let stop$ = searchto$(zplport, "^XZ")
     270  end if
     1 rem ********************************************************
     1 rem print current data value, prompt user to replace data
     1 rem ********************************************************
     280  print ""
     290  print "Current number in format is " & counter$
     300  print "Please enter new number (type EXIT to end) ";
     310  input new_counter$
     320  if new_counter$ = "EXIT" then
     330    print "Program ending"
     340    end
     350  else
     360    print #zplport:"^XA^DFE:FORMAT.ZPL^FS";
     370    print #zplport:"^FX" & new_counter$ & "^FS^XZ "
     380  end if
     390 loop