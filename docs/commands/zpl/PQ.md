# ^PQ




ZPL Commands


The `^PQ` command gives control over several printing operations. It controls the number of labels to print,
the number of labels printed before printer pauses, and the number of replications of each serial number.


**Print Quantity**

**Format:** `^PQq,p,r,o,e`












|Parameters|Details|
|---|---|
|`q =` total quantity of<br>labels to print|**Values:** `1` to`99,999,999`<br>**Default:** `1`|
|`p =` pause and cut value<br>(labels between pauses)|**Values:** `1` to`99,999,999`<br>**Default:** `0` (no pause)|
|`r =` replicates of each<br>serial number|**Values:** `0` to`99,999,999` replicates<br>**Default: :** `0` (no replicates)|
|`o =` override pause count|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|
|`e =` cut on error label<br>(RFID void is an error<br>label)|**Values:**<br>`N =` no - if a cutter is installed, a cut will be made after a voided RIFD label<br>ONLY if a cut would be made after the non-voided label and this was the<br>last retry.<br>`Y =` yes - if a cutter is installed, a cut will be made after ANY voided RFID<br>label.<br>**Default:** Y|



If the `o` parameter is set to `Y`, the printer cuts but does not pause, and the printer does **not** pause after
every group count of labels has been printed. With the `o` parameter set to `N` (default), the printer pauses
after every group count of labels has been printed.


**Example:** This example shows the control over print operations:

`^PQ50,10,1,Y` : This example prints a total of 50 labels with one replicate of each serial number. It prints
the total quantity in groups of 10, but does not pause after every group.

`^PQ50,10,1,N` : This example prints a total of 50 labels with one replicate of each serial number. It prints
the total quantity in groups of 10, pausing after every group.


323