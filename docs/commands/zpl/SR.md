# ^SR




ZPL Commands


The `^SR` command allows you to set the printhead resistance.


**Set Printhead Resistance**

**Format:** `^SR####`






|Parameters|Details|
|---|---|
|`#### =` resistance value<br>(four-digit numeric value)|**Values:** `0488` to`1175`<br>**Default:** last permanently saved value|



**Comments:** To avoid damaging the printhead, this value should be less than or equal to the value shown
on the printhead being used. Setting a higher value could damage the printhead.


**NOTE:** New printer models automatically set head resistance.


347