# ~JF




ZPL Commands


**Set Battery Condition**


There are two low battery voltage levels sensed by the PA/PT400™ printers. When battery voltage goes
below the first level, the green LED begins flashing as a warning but printing continues. When this warning
occurs, it is recommended to recharge the battery.


As printing continues, a second low voltage level is reached. At this point, both green and orange LEDs
flash as a warning, and printing automatically pauses.


When pause on low voltage is active (~JFY) and the battery voltage level falls below the second low
voltage level, printing pauses and an error condition is displayed as an indication that the printer should be
plugged into the battery charger. By pressing FEED, printing continues on a label-by-label basis, but there
is a high risk of losing label format information due to the continued decrease of battery voltage.


When pause on low voltage is not active (~JFN), and the battery voltage level falls below the second
low voltage level, printing continues and the orange LED remains off. If the battery voltage continues to
decrease, label information could be lost and cause the printer to stop operating. This option should be
selected only when the printer is connected to the Car Battery Adapter. From time to time the printer might
sense that battery voltage is below the first low voltage level, but due to the continuous recharging of the
car battery, further loss of battery voltage is not a concern and printing continues.


If this option is not selected when using the Car Battery Adapter, you might need to press FEED to take the
printer out of Pause Mode and print each label.

**Format:** `~JFp`

|Parameters|Details|
|---|---|
|`p =` pause on low voltage|**Values:** `Y` (pause on low voltage) or`N` (do not pause)<br>`N` is suggested when the printer is powered by the Car Battery Adapter.<br>**Default:** `Y`|



257