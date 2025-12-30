# ^MC




ZPL Commands


In normal operation, the bitmap is cleared after the format has been printed. The `^MC` command is used to
retain the current bitmap. This applies to current and subsequent labels until cleared with `^MCY` .


**Map Clear**

**Format:** `^MCa`


**IMPORTANT:** To produce a label template, `^MC` must be used with `^FV` .

|Parameters|Details|
|---|---|
|`a =` map clear|**Values:** `Y` (clear bitmap) or`N` (do not clear bitmap)<br>**Initial Value at Power Up:** `Y`|



**Comments:** The `^MC` command retains the image of the current label after formatting. It appears in the
background of the next label printed.


300