# device.frontpanel.xml



This command retrieves the current content of the front panel in an XML format.


**Getvar**


To retrieve the file that determines the representation of the front panel:

```
       ! U1 getvar "device.frontpanel.xml"

```

**Example**

In this example, the `getvar` shows the status of the LEDs and the two lines of the front panel in XML
formatted text. The text below is formatted for easy reading. When you use this command the response will
not contain line feeds.

```
       ! U1 getvar "device.frontpanel.xml"
       <FRONT-PANEL>
       <LCD>
       <LINE1>PRINTER READY</LINE1>
       <LINE2>V53.16.0</LINE2>
       </LCD>
       <LEDS>
       <PAUSE-LED>STEADY-OFF</PAUSE-LED>
       <DATA-LED>STEADY-OFF</DATA-LED>
       <ERROR-LED>STEADY-OFF</ERROR-LED>
       </LEDS>
       </FRONT-PANEL>

```

702


SGD Printer Commands