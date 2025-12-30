# ^JJ




ZPL Commands


The `^JJ` command allows you to control an online verifier or applicator device.


**Set Auxiliary Port**

**Format:** `^JJa,b,c,d,e,f`











|Parameters|Details|
|---|---|
|`a =` operational mode for<br>auxiliary port|**Values:**<br>`0 =` off<br>`1 =` reprint on error—the printer stops on a label with a verification error.<br>When**PAUSE** is pressed, the label reprints (if`^JZ` is set to reprint). If a bar<br>code is near the upper edge of a label, the label feeds out far enough for<br>the bar code to be verified and then backfeeds to allow the next label to be<br>printed and verified.<br>`2 =` maximum throughput—the printer stops when a verification error is<br>detected. The printer starts printing the next label while the verifier is still<br>checking the previous label. This mode provides maximum throughput, but<br>does not allow the printer to stop immediately on a label with a verification<br>error.<br>**Default:** `0`|
|`b =` application mode|**Values:**<br>`0 =` off<br>`1 =` End Print signal normally high, and low only when the printer is moving<br>the label forward.<br>`2 =` End Print signal normally low, and high only when the printer is moving<br>the label forward.<br>`3 =` End Print signal normally high, and low for 20 ms when a label has<br>been printed and positioned.<br>`4 =` End Print signal normally low, and high for 20 ms when a label has<br>been printed and positioned.<br>**Default:** `0`<br>**NOTE:** The Set/Get/Do commanddevice.applicator.end_print on<br>page 665controls the same setting as the`b` parameter.|
|`c =` application mode<br>start signal print|**Values:**<br>`p =` Pulse Mode – Start Print signal must be de-asserted before it can be<br>asserted for the next label.<br>`l =` Level Mode – Start Print signal does not need to be de-asserted to<br>print the next label. As long as the Start Print signal is low and a label is<br>formatted, a label prints.<br>**Default:** `0`|


266


ZPL Commands







|Parameters|Details|
|---|---|
|`d =` application label<br>error mode|**Values:**<br>`e =` error mode—the printer asserts the**Service Required** signal (svce_req<br>- pin 10) on the application port, enters into Pause Mode, and displays an<br>error message on the LCD.<br>`f =` Feed Mode—a blank label prints when the web is not found where<br>expected to sync the printer to the media.<br>**Default:** `f`|
|`e =` reprint mode|**Values:**<br>`e =` enabled—the last label reprints after the signal is asserted. If a label is<br>canceled, the label to be reprinted is also canceled. This mode consumes<br>more memory because the last printed label is not released until it reprints.<br>`d =` disabled—printer ignores the Reprint signal.<br>**Default:** `d`|
|`f =` ribbon low mode|**Values:**<br>`e` = enabled – printer warning issued when ribbon low.<br>`d` = disabled – printer warning not issued when ribbon low.<br>**Default:** `e`|


267