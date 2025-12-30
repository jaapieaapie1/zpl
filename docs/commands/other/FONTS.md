# Fonts and Barcodes


**Fonts and Barcodes**


This section provides information about different fonts (type faces) and barcodes that can be used with the
printer.

## **Standard Printer Fonts**


Most Zebra printers come standard with 15 bitmapped fonts and one scalable font.


Additional downloadable bitmapped and scalable fonts are also available. Character size and density (how
dark it appears) depend on the density of the printhead and the media used.


**Figure 22** Examples of the Standard Printer Fonts


1562


Fonts and Barcodes


To use one of these fonts, you must either use the change alphanumeric default font command ( `^CF` ) or
specify an alphanumeric field command ( `^A` ).

The standard Zebra character set is Code 850 for character values greater than 20 HEX. There are six HEX
character values below 20 HEX that are also recognized. The figure below shows how these character
values are printed.


**NOTE:** Unidentified characters should default to a space.


**Figure 23** Recognized HEX Values below 20 HEX

## **Proportional and Fixed Spacing**


Proportional spacing is different than fixed spacing. In Table 29  Intercharacter Gap and Baseline
Parameters on page 1564, the intercharacter gap (ICG), the space between characters, is constant for


1563


Fonts and Barcodes


fonts A through H, which means that the spacing between all characters is the same. For example, the
spacing between the letters `MW` is the same as between the letters `IE` .


**Figure 24** Fixed Space Fonts Proportion Example


The baseline is the imaginary line on which the bottom (base) of all characters (except any descenders)
rest. The area between the baseline and the bottom of the matrix is used for any character “descenders.”
Baseline numbers define where the baseline is located in relationship to the top of the matrix. For example,
the baseline for font “E” is 23 dots down from the top of the matrix.


**Table 29** Intercharacter Gap and Baseline Parameters

|Font|H x W (in dots)|Type|Intercharacter Gap (in dots)|Baseline(in dots)|
|---|---|---|---|---|
|A|9 x 5|U-L-D|1|7|
|B|11 x 7|U|2|11|
|C,D|18 x 10|U-L-D|2|14|
|E|28 x 15|OCR-B|5|23|
|F|26 x 13|U-L-D|3|21|
|G|60 x 40|U-L-D|8|48|
|H|21 x 13|OCR-A|6|21|
|GS|24 x 24|SYMBOL|PROPORTIONAL|3 x HEIGHT/4|
|0|DEFAULT: 15 x 12||PROPORTIONAL|3 x HEIGHT/4|



1564


Fonts and Barcodes

## **Scalable Versus Bitmapped Fonts**


For scalable fonts, setting the height and width equally produces characters that appear the most
balanced. Balanced characters are pleasing to the eye because actual height and width are approximately
equal to each other. This is achieved through the use of a smooth-scaling algorithm in the printer.


For bitmapped fonts, this balancing is built into the font. In actuality, the height of a bitmap font is slightly
larger than the width. Bitmap fonts are always at the maximum size of the character’s cell.

## **Font Matrices**


This section lists the font matrices.


Type Key


U = Uppercase, L = Lowercase, D = Descenders


**Table 30** 6 dot/mm Printhead







|Font|Matrix|Type|Character Size|Col5|Col6|Col7|
|---|---|---|---|---|---|---|
|**Font**|**HxW (in dots)**|**HxW (in dots)**|**HxW (in in.)**|**Char./in.**|**HxW (in mm)**|**Char. /mm**|
|A|9 x 5|U-L-D|0.059 x 0.039|25.4|1.50 x 0.99|1.01|
|B|11 x 7|U|0.072 x 0.059|16.9|1.82 x 1.50|0.066|
|C, D|18 x 10|U-L-D|0.118 x 0.079|12.7|2.99 x 2.00|0.05|
|E|21 x 10|OCR-B|0.138 x 0.085|11.7|3.50 x 2.16|0.46|
|F|26 x 13|U-L-D|0.170 x 0.105|9.53|4.32 x 2.67|0.37|
|G|60 x 40|U-L-D|0.394 x 0.315|3.18|10.0 x 8.00|0.125|
|H|17 x 11|OCR-A|0.111 x 0.098|10.2|2.81 x 2.48|0.40|
|GS|24 x 24|SYMBOL|0.157 x 0.157|6.35|3.98 x 3.98|0.251|
|0|Default: 15 x 12||||||


**Table 31** 8 dot/mm (203 dpi) Printhead







|Font|Matrix|Type|Character Size|Col5|Col6|Col7|
|---|---|---|---|---|---|---|
|**Font**|**HxW (in dots)**|**HxW (in dots)**|**HxW (in in.)**|**Char./in.**|**HxW (in mm)**|**Char. /mm**|
|A|9 X 5|U-L-D|0.044 x 0.030|33.3|1.12 x 0.76|1.31|
|B|11 X 7|U|0.054 x 0.044|22.7|1.37 x 1.12|0.89|
|C, D|18 X 10|U-L-D|0.089 x 0.059|16.9|2.26 x 1.12|0.66|
|E|28 x 15|OCR-B|0.138 x 0.098|10.2|3.50 x 2.49|0.40|
|F|26 x 13|U-L-D|0.128 x 0.079|12.7|3.25 x 2.00|0.50|
|G|60 x 40|U-L-D|0.295 x 0.197|4.2|7.49 x 5.00|0.167|
|H|21 x 13|OCR-A|0.103 x 0.093|10.8|2.61 x 2.36|0.423|
|GS|24 x 24|SYMBOL|0.118 x 0.118|8.5|2.99 x 2.99|0.334|


1565


Fonts and Barcodes


**Table 31** 8 dot/mm (203 dpi) Printhead (Continued)







|Font|Matrix|Type|Character Size|Col5|Col6|Col7|
|---|---|---|---|---|---|---|
|**Font**|**HxW (in dots)**|**HxW (in dots)**|**HxW (in in.)**|**Char./in.**|**HxW (in mm)**|**Char. /mm**|
|P|20 x 18|U-L-D|0.098 x 0.089|N/A|2.50 x 2.25|N/A|
|Q|28 x 24|U-L-D|0.138 x 0.118|N/A|3.50 x 3.00|N/A|
|R|35 x 31|U-L-D|0.172 x 0.153|N/A|4.38 x 3.88|N/A|
|S|40 x 35|U-L-D|0.197 x 0.172|N/A|5.00 x 4.38|N/A|
|T|48 x 42|U-L-D|0.236 x 0.207|N/A|6.00 x 5.25|N/A|
|U|59 x 53|U-L-D|0.290 x 0.261|N/A|7.38 x 6.63|N/A|
|V|80 x 71|U-L-D|0.394 x 0.349|N/A|10.00 x 8.88|N/A|
|0|Default: 15 x 12|U-L-D|Scalable||Scalable||


**Table 32** 12 dot/mm (300 dpi) Printhead







|Font|Matrix|Type|Character Size|Col5|Col6|Col7|
|---|---|---|---|---|---|---|
|**Font**|**HxW (in dots)**|**HxW (in dots)**|**HxW (in in.)**|**Char./in.**|**HxW (in mm)**|**Char. /mm**|
|A|9 X 5|U-L-D|0.030 x 0.020|50.8|0.75 x 0.50|2.02|
|B|11 X 7|U|0.036 x 0.030|33.8|0.91 x 0.75|1.32|
|C, D|18 X 10|U-L-D|0.059 x 0.040|25.4|1.50 x 1.00|1.00|
|E|42 x 20|OCR-B|0.138 x 0.085|23.4|1.75 x 1.08|0.92|
|F|26 x 13|U-L-D|0.085 x 0.053|19.06|2.16 x 1.34|0.74|
|G|60 x 40|U-L-D|0.197 x 0.158|6.36|5.00 x 4.00|0.25|
|H|34 x 22|OCR-A|0.111 x 0.098|10.20|2.81 x 2.48|0.40|
|GS|24 x 24|SYMBOL|0.079 x 0.079|12.70|1.99 x 1.99|0.52|
|P|20 x 18|U-L-D|0.067 x 0.060|N/A|1.69 x 1.52|N/A|
|Q|28 x 24|U-L-D|0.093 x 0.080|N/A|2.37 x 2.03|N/A|
|R|35 x 31|U-L-D|0.117 x 0.103|N/A|2.96 x 2.62|N/A|
|S|40 x 35|U-L-D|0.133 x 0.177|N/A|3.39 x 2.96|N/A|
|T|48 x 42|U-L-D|0.160 x 0.140|N/A|4.06 x 3.56|N/A|
|U|59 x 53|U-L-D|0.197 x 0.177|N/A|5.00 x 4.49|N/A|
|V|80 x 71|U-L-D|0.267 x 0.237|N/A|6.77 x 6.01|N/A|
|0|Default: 15 x 12|U-L-D|Scalable||Scalable||


1566


Fonts and Barcodes


**Table 33** 24 dot/mm (600 dpi) Printhead






|Font|Matrix|Type|Character Size|Col5|Col6|Col7|
|---|---|---|---|---|---|---|
|**Font**|**HxW (in dots)**|**HxW (in dots)**|**HxW (in in.)**|**Char./in.**|**HxW (in mm)**|**Char. /mm**|
|A|9 X 5|U-L-D|0.015 x 0.010|100.00|0.38 x 0.25|4.00|
|B|11 X 7|U|0.018 x 0.015|66.66|0.46 x 0.38|2.60|
|C, D|18 X 10|U-L-D|0.030 x 0.020|50.00|0.77 x 0.51|2.0|
|E|42 x 20|OCR-B|0.137 x 0.087|11.54|3.47 x 2.20|0.45|
|F|26 x 13|U-L-D|0.043 x 0.027|37.5|1.10 x 0.68|1.50|
|G|60 x 40|U-L-D|0.100 x 0.080|12.50|2.54 x 2.04|0.50|
|H|34 x 22|OCR-A|0.100 x 0.093|10.71|2.54 x 2.37|0.42|
|GS|24 x 24|SYMBOL|0.040 x 0.040|25.00|1.02 x 1.02|1.00|
|P|20 x 18|U-L-D|0.067 x 0.060|N/A|1.69 x 1.52|N/A|
|Q|28 x 24|U-L-D|0.093 x 0.080|N/A|2.37 x 2.03|N/A|
|R|35 x 31|U-L-D|0.117 x 0.103|N/A|2.96 x 2.62|N/A|
|S|40 x 35|U-L-D|0.133 x 0.117|N/A|3.39 x 2.96|N/A|
|T|48 x 42|U-L-D|0.160 x 0.140|N/A|4.06 x 3.56|N/A|
|U|59 x 53|U-L-D|0.197 x 0.177|N/A|5.00 x 4.49|N/A|
|V|80 x 71|U-L-D|0.267 x 0.237|N/A|6.77 x 6.01|N/A|
|0|Default: 15 x 12|U-L-D|Scalable||Scalable||


## **Barcodes**



Every barcode contains data made up of a sequence of light spaces and dark bars that represent letters,
numbers, or other graphic characters. 

The usable characters differ among the various kinds of bar codes. Each barcode section in the ZPL
Commands provides a table of applicable characters. Start and stop characters and check digits are used
by many, but not all, barcodes. These will be indicated in the specific barcode explanations.


Zebra printers can print the following kinds of barcodes:


1567


Fonts and Barcodes



|Barcode modulus “X” dimensions<br>• Picket fence (non-rotated) orientation:<br>• 203 dpi = 0.0049 in. mil to 0.049 in.<br>• 300 dpi = 0.0033 in. mil to 0.033 in.<br>• Ladder (rotated) orientation:<br>• 203 dpi = 0.0049 in. mil to 0.049 in.<br>• 300 dpi = 0.0039 in. mil to 0.039 in.|Linear barcodes<br>• Codabar<br>• Code 11<br>• Code 39<br>• Code 93<br>• Code 128 with subsets A/B C and UCC Case<br>Codes<br>ISBT-128<br>•<br>• UPC-A<br>• UPC-E<br>• EAN-8<br>EAN-13<br>•<br>• UPC and EAN 2 or 5 digit extensions<br>• Planet Code<br>• Plessey<br>• Postnet<br>• Standard 2 of 5<br>• Industrial 2 of 5<br>• Interleaved 2 of 5<br>• LOGMARS<br>• MSI<br>• GS1 DataBar Omnidirectional|
|---|---|
|Two-dimensional barcodes<br>•<br>Aztec<br>•<br>Code 49<br>•<br>Maxi Code<br>•<br>TLC39<br>•<br>PDF-417<br>•<br>QR Code<br>•<br>Codablock<br>•<br>DataMatrix<br>•<br>Micro-PDF417|Two-dimensional barcodes<br>•<br>Aztec<br>•<br>Code 49<br>•<br>Maxi Code<br>•<br>TLC39<br>•<br>PDF-417<br>•<br>QR Code<br>•<br>Codablock<br>•<br>DataMatrix<br>•<br>Micro-PDF417|
|Barcode ratios<br>•<br>2:1<br>•<br>7:3<br>•<br>5:2<br>•<br>3:1|Barcode ratios<br>•<br>2:1<br>•<br>7:3<br>•<br>5:2<br>•<br>3:1|

## **Basic Format for Bar Codes**

















The basic format for bar codes is quiet zone, start character, data, check digit, stop character, and quiet
zone. Not all bar codes require each of these elements.


Every bar code requires a quiet zone. A quiet zone (sometimes called a “clear area”) is an area adjacent
to the machine-readable symbols that ensure proper reading (decoding) of the symbols. No printing is
permissible within this area. Preprinted characters, borders, and background color are acceptable if they
are invisible to the reading device; these are used in some applications but restrict the type of reading
device that can be used. The size of the quiet zone depends on the size of bar widths (usually 10 times the
width of the narrow bar).


1568


Fonts and Barcodes


**Figure 25** Quiet Zone in a Bar Code

## **Barcode Field Instructions**


To create a barcode, a barcode field command must be contained in the label format.


Table 34  Barcode Field Commands on page 1569 shows the barcode field commands. The number in
brackets denotes the print ratio. Each command produces a unique barcode.


**IMPORTANT:** (*) for Fixed Printing Ratio means that the ratio between the width of the bars in the
code is a fixed standard and cannot be changed.


As another reference to the barcode field commands ratio, see Table 6  Module Width Ratios in Dots on
page 148.


**Table 34** Barcode Field Commands

|ZPL Command|Command Description|Ratio|
|---|---|---|
|`^B0`|Aztec Barcode Parameters|[Fixed]|
|`^B1`|Code 11 (USD-8)|[2.0 - 3.0]|
|`^B2`|Interleaved 2 of 5|[2.0 - 3.0]|
|`^B3`|Code 39 (USD-3 and 3 of 9)|[2.0 - 3.0]|
|`^B4`|Code 49 (*)|[Fixed]|
|`^B5`|Planet Code Barcode|[Fixed]|
|`^B7`|PDF417 (*)|[Fixed]|
|`^B8`|EAN-8 (*)|[Fixed]|
|`^B9`|UPC-E|[Fixed]|
|`^BA`|Code 93 (USS-93)(*)|[Fixed]|
|`^BB`|CODABLOCK A, E, F (*)|[Fixed]|
|`^BC`|Code 128 (USD-6) (*)|[Fixed]|
|`^BD`|UPS MaxiCode (*)|[Fixed]|
|`^BE`|EAN-13|[Fixed]|
|`^BF`|Micro-PDF417|[Fixed]|



1569


Fonts and Barcodes


**Table 34** Barcode Field Commands (Continued)

|ZPL Command|Command Description|Ratio|
|---|---|---|
|`^BI`|Industrial 2 of 5|[2.0 - 3.0]|
|`^BJ`|Standard 2 of 5|[2.0 - 3.0]|
|`^BK`|ANSI Codabar (USD-4 and 2 of 7)|[2.0 - 3.0]|
|`^BL`|LOGMARS|[2.0 - 3.0]|
|`^BM`|MSI|[2.0 - 3.0]|
|`^BO`|Aztec Barcode Parameters|[Fixed]|
|`^BP`|Plessey|[2.0 - 3.0]|
|`^BQ`|QR Code (*)|[Fixed]|
|`^BR`|GS1 Databar (formerly RSS)|[Fixed]|
|`^BS`|UPC/EAN Extensions (*)|[Fixed]|
|`^BU`|UPC-A (*)|[Fixed]|
|`^BX`|Data Matrix (*)|[Fixed]|
|`^BZ`|PostNet (*), USPS Intelligent Mail, and Planet barcodes|[Fixed]|



Additionally, each barcode field command can be issued with a definition parameter string. The parameter
string defines field rotation, height, and interpretation line status for all barcodes. For some barcodes, the
parameter string also sets a check digit, start character, and/or stop character. Use the definition parameter
string to command the printer to print barcodes of appropriate heights and densities that conform to the
specifications of the application.


The use of the parameter string is optional because all parameters have default values. If the default
values for all of the barcode parameters suit the application, then only the barcode command needs to be
entered.


Parameters in barcode field commands are “position specific.” If a value (other than the default value) is
manually entered for one parameter the ZPL II delimiter character (a comma) must be used to mark the
position of the preceding parameters in the string.


To change just the third parameter, enter two commas and then the value for the third parameter. The
default values will be automatically used for the first and second parameters.

## **Bar Code Command Groups**


Bar code commands are organized into four groups.


Each group represents a type of bar code. The following tables identify the groups and the bar codes they
contain:


**Table 35** Numeric Only Bar Codes

|ZPL Command|Command Description|
|---|---|
|`^B0`|Aztec Bar Code Parameters|
|`^B1`|Code 11|



1570




Fonts and Barcodes


**Table 35** Numeric Only Bar Codes (Continued)

|ZPL Command|Command Description|
|---|---|
|`^B5`|Planet Code Bar Code|
|`^BI`|Industrial 2 of 5|
|`^BJ`|Standard 2 of 5|
|`^BK`|ANSI Codabar (or NW-7)|
|`^BM`|MSI|
|`^BO`|Aztec Bar Code Parameters|
|`^BP`|Plessey|
|`^BZ`|PostNet (*), USPS Intelligent Mail, and Planet bar codes|



**Table 36** Retail Labeling Bar Codes

|ZPL Command|Command Description|
|---|---|
|`^B0`|Aztec Bar Code Parameters|
|`^B8`|EAN-8|
|`^B9`|UPC-E|
|`^BE`|EAN-13|
|`^BO`|Aztec Bar Code Parameters|
|`^BS`|UPC/EAN extensions|
|`^BU`|UPC-A|



**Table 37** Alphanumeric Bar Codes

|ZPL Command|Command Description|
|---|---|
|`^B0`|Aztec Bar Code Parameters|
|`^B3`|Code 39|
|`^BA`|Code 93|
|`^BC`|Code 128|
|`^BL`|LOGMARS|
|`^BO`|Aztec Bar Code Parameters|



**Table 38  Two-Dimensional Bar Codes**

|ZPL Command|Command Description|
|---|---|
|`^B0`|Aztec Bar Code Parameters|
|`^B4`|Code 49|
|`^B7`|PDF417|



1571


Fonts and Barcodes


**Table 38  Two-Dimensional Bar Codes** (Continued)

|ZPL Command|Command Description|
|---|---|
|`^BB`|CODABLOCK|
|`^BD`|UPS MaxiCode|
|`^BF`|MicroPDF417|
|`^BQ`|QR Code|
|`^BO`|Aztec Bar Code Parameters|
|`^BR`|GS1 Databar (formerly RSS)|
|`^BT`|TLC39|
|`^BX`|Data Matrix|



1572


# **Mod 10 and Mod 43** **Check Digits**

**Mod 10 and Mod 43 Check Digits**


This section provides information about Mod 10 and Mod 43 check digits.

## **Mod 10 Check Digit**


This section lists the calculations for determining the Mod 10 Check Digit character.


The calculations for determining the Mod 10 Check Digit character are as follows:


**1.** Start at the first position and add the value of every other position together.


0 + 2 + 4 + 6 + 8 + 0 = 20


**2.** The result of Step 1 is multiplied by 3.


20 x 3 = 60


**3.** Start at the second position and add the value of every other position together.


1 + 3 + 5 + 7 + 9 = 25


**4.** The results of steps 2 and 3 are added together.


60 + 25 = 85


**5.** The check character (12th character) is the smallest number which, when added to the result in step 4,
produces a multiple of 10.


85 + X = 90 (next higher multiple of 10)


X = 5 Check Character


This bar code illustrates the above example. The digit on the right (5) is the check digit.


1573


Mod 10 and Mod 43 Check Digits

## **Mod 43 Check Digit**


This section lists the calculations for determining the Mod 43 check Digit character.


The calculations for determining the Mod 43 check Digit character are as follows:


Each character in the Code 39 character set has a specific value, as follows:

|0=0|B=11|X=33|
|---|---|---|
|1=1|C=12|Y=34|
|2=2|D=13|Z=35|
|3=3|E=14|- =36|
|4=4|F=15|. =37|
|5=5|G=16|Space = 38|
|6=6|H=17|$=39|
|7=7|I=18|/=40|
|8=8|J=19|+=41|
|9=9|K=20|%=42|
|A=10|L=21||



Example


Data string 2345ABCDE/


**1.** Add the sum of all the character values in the data string. Using the chart above, the sum of the
character values is as follows:


1 + 2 + 3 + 4 + 5 + 10 + 11 + 12 + 13 + 14 + 40 =115


**2.** Divide the total by 43. Keep track of the remainder.


115/43 = 2 Remainder is 29


1574


Mod 10 and Mod 43 Check Digits


**3.** The “check digit” is the character that corresponds to the value of the remainder.


Remainder = 29


29 is the value for the letter T.


T is the check digit.


Below is a bar code that illustrates the example. The character on the right, T, is the check digit.


1575