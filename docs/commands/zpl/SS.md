# ^SS




ZPL Commands


The `^SS` command is used to change the values for media, web, ribbon, and label length set during the
media calibration process. The media calibration process is described in your specific printer’s user’s
guide.


**Set Media Sensors**

**Format:** `^SSw,m,r,l,m2,r2,a,b,c`



|Parameters|Details|
|---|---|
|`w =` web (3-digit value)|**Values:** `000` to`100`<br>**Default:** the value is shown on the media sensor profile or configuration<br>label|
|`m =` media (3-digit value)|**Values:** `000` to`100`<br>**Default:** the value shown on the media sensor profile or configuration label|
|`r =` ribbon (3-digit value)|**Values:** `000` to`100`<br>**Default:** the value is shown on the media sensor profile or configuration<br>label|
|`l =` label length (in dots,<br>four-digit value)|**Values:** `0001` to`32000`<br>**Default:** value calculated in the calibration process|
|`m2 =` intensity of media<br>LED (3-digit value)|**Values:** `000` to`255`<br>**Default:** value calculated in the calibration process|
|`r2 =` intensity of ribbon<br>LED (3-digit value)|**Values:** `000` to`255`<br>**Default:** value calculated in the calibration process|
|`a =` mark sensing (3-digit<br>value)|**Values:** `000` to`100`<br>**Default:** value calculated in the calibration process|
|`b =` mark media sensing<br>(3-digit value)|**Values:** `000` to`100`<br>**Default:** value calculated in the calibration process|
|`c =` mark LED sensing (3-<br>digit value)|**Values:** `000` to`255`<br>**Default:** value calculated in the calibration process|


**Example**





Below is an example of a media sensor profile. Notice the numbers from 000 to 100 and where the words
WEB, MEDIA, and RIBBON appear in relation to those numbers. Also, notice the black vertical spike. This
represents where the printer sensed the transition from media to web to media.


348


ZPL Commands


The media and sensor profiles produced vary in appearance from printer to printer.

**Comments:** The `m2` and `r2` parameters have no effect in Stripe® **S** -300 and **S** -500 printers. This command
is ignored on the HC100™ printer. Maximum values for parameters depend on which printer platform is
being used.


349