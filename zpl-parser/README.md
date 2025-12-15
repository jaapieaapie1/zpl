# ZPL Parser
The responsibility of this module is to take the tokens from the ZPL tokenizer and parse these tokens into usable command types.  

## Limitations
To keep things simple, for now we will not be supporting the entire ZPL spec.  
Most ZPL commands are rarely used and thus not very relevant for our rendering purposes.  
For now we will support the following commands:
- ^XA - Start indication
- ^XZ - End indication
- ^FS - Field Seperator
- ^FO - Field origin
- ^LH - Label Home
- ^LT - Label Top
- ^PW - Print Width
- ^LL - Label Length
- ^A - Font
- ^CF - Change default font
- ^FD - Field Data
- ^FB - Field Block
- ^GB - Graphic Box
- ^GF - Graphic Field
- ^BC - Code128 Barcode
- ^BY - Barcode Field Default
- ^BQ - QR Code
- ^B3 - Code 39 Barcode
- ^FT - Field Typeset
- ^CI - Change International Font
- ^PQ - Print quantity
- ^MD - Media Darkness
- ^FX - Comment
- ^GC - Graphic Circle
- ^GD - Graphic Diagonal Line

This list is semi ordered on priority.

## Concern
This project should only concern itself with turning tokens into command types.  
It must not keep context. It should not decide defaults. It must return errors when commands are not valid.
It must skip over but report unknown commands. It must keep command order.
