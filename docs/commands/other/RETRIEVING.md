# Retrieving WML Content from the Printer using the file.type Command



Use this procedure to retrieve content from the printer using the file.type command.

It is possible to retrieve .wml file content from the printer using the `"file.type"` SGD command. To do
this, open a terminal emulation connection to the printer and issue the command. For example, to retrieve
the contents of the INDEX.WML file, use the following command:

```
       ! U1 setvar "file.type" "E:INDEX.WML"

```

**NOTE:** The `file.type` command is case sensitive – if the file is stored on the printer as
`INDEX.WML`, the command must use that same case. Additionally, you should note that `.nrd`
files are treated as confidential – they cannot be retrieved from the printer.