# DIR




ZBI Commands


This command, with no filter included, prompts the printer to list all of the ZBI programs residing in all
printer memory locations.


Including a filter signals the printer to limit the search; including a drive location signals the printer to
search in only one location.


Asterisks (*) are used as wild cards. A wild card (*) finds every incidence of a particular request. The
example here, `DIR "B:*.BAS"`, signals the printer to search for every file with a `.BAS` extension in B:
memory.


**Format**
```
   DIR [<filter$>]
```

**Parameters**

`[<filter$>]` = the name of the file to be accessed (optional). Drive location and file name must
be in quotation marks.

Default = `"*:*.bas"`


**NOTE:** Quotes must be around what you are doing.


**Example**
N/A

**Comments**
This is an interactive command that takes effect as soon as it is received by the printer.


512