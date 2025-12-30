# apl.settings



Link-0S v6.3 and later include PDF Direct, which enables a printer to directly process PDF files received
for printing. When using PDF Direct, sending a new `apl.settings` command overrides all previous
commands. It is possible to send one command to set the multiple values.


By default, PDF Direct is disabled. To enable PDF Direct, use the command:

```
       ! U1 setvar "apl.enable" "pdf"

```

**Setvar**


To configure PDF Direct:

```
       ! U1 setvar "apl.settings" "value"

```

**Values**

`"dither"`, `"scale"`, `"scale-to-fit"`, `"no-varlen"`, `"orient"`, `"zpl-attach"`


**NOTE:** Multiple values can be specified in an `apl.settings` command.


**Dithering**

The default is off. To turn on, set `apl.settings` to include `dither` . For example:

```
       ! U1 setvar "apl.settings" "dither"

```

**Scaling**


**Auto-Scale/Rotate**

Default is off. To turn on, set `apl.settings` to include `scale-to-fit` . This setting modifies
the size of the text or image to fit the media. The `scale-to-fit` setting takes precedence over
`scale=WxH`, regardless of their order in a command.

For example:

```
          ! U1 setvar "apl.settings" "scale-to-fit no-varlen orient=N"

```

**Scale Factor**

Default is off. To turn on, set `apl.settings` to include `scale=` WxH. The W and H values are
percentages that range from 1 to 100.


For example:

```
          ! U1 setvar "apl.settings" "scale=50x50"

```

633


SGD Printer Commands


**Variable Length**

Default is on when `ezpl.media_type` is set to `continuous` . To turn off, set `apl.settings` to
include `no-varlen` . Specify `no-varlen` to use `scale-to-fit` with continuous media of a fixed
or pre-configured length. The `varlen` setting disables `scale-to-fit`, use scale factor instead.

For example:

```
   ! U1 setvar "apl.settings" "scale-to-fit no-varlen scale=50x50"

```

**NOTE:** Use the Scale Factor and Auto-Scale with caution. Scaling can result in
unscannable barcodes.


**Page Orientation**


**Page Orientation**


Default is ZPL "Inverted" (top-first), to handle multi-page documents. To change, set
`apl.settings` to include `orient=N` . Possible values are N (normal) or I (inverted).


**Advanced Options**


**ZPL Attach**

All `"^XA..^XZ"` strings (uppercase letters) are concatenated and appended to the next PDF
document (all pages), after which the strings are cleared. Use this setting to prepend raw ZPL data
as a header to PDF documents. The `"^XA..^XZ"` data will only be used for **zpl-attach** to PDF
documents and not printed normally as-is.

Note that `"^xa..^xz"` strings (lowercase letters), and other data including SGDs, will continue to
be passed through when received.

The default value is off. To enable, set `apl.settings` to include `"zpl-attach"` .


**Getvar**


To return the current PDF Direct settings:

```
! U1 getvar "apl.settings"

```

634


SGD Printer Commands