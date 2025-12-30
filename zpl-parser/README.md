# ZPL Parser
The responsibility of this module is to take the tokens from the ZPL tokenizer and parse these tokens into usable command types.  

## Limitations
To keep things simple, for now we will not be supporting the entire ZPL spec.  
Most ZPL commands are rarely used and thus not very relevant for our rendering purposes.  
For now we will support the following commands:
- [x] ^XA - Start indication
- [x] ^XZ - End indication
- [x] ^FS - Field Seperator
- [x] ^FO - Field origin
- [x] ^LH - Label Home
- [x] ^LT - Label Top
- [x] ^PW - Print Width
- [x] ^LL - Label Length
- [x] ^A - Font
- [x] ^CF - Change default font
- [x] ^FD - Field Data
- [x] ^FB - Field Block
- [x] ^GB - Graphic Box
- [x] ^GF - Graphic Field
- [x] ^BC - Code128 Barcode
- [x] ^BY - Barcode Field Default
- [x] ^BQ - QR Code
- [x] ^B3 - Code 39 Barcode
- [x] ^FT - Field Typeset
- [x] ^CI - Change International Font
- [x] ^PQ - Print quantity
- [x] ^MD - Media Darkness
- [x] ^FX - Comment
- [x] ^GC - Graphic Circle
- [x] ^GD - Graphic Diagonal Line

This list is semi ordered on priority.

## Concern
This project should only concern itself with turning tokens into command types.  
It must not keep context. It should not decide defaults. It must return errors when commands are not valid.
It must skip over but report unknown commands. It must keep command order.
