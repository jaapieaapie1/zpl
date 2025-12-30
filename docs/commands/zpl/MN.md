# ^MN




ZPL Commands


This command specifies the media type being used and the black mark offset in dots.


**Media Tracking**


This bulleted list shows the types of media associated with this command:


- Continuous Media – this media has no physical characteristic (such as a web, notch, perforation, black
mark) to separate labels. Label length is determined by the `^LL` command.

- Continuous Media, variable length – same as Continuous Media, but if portions of the printed label fall
outside of the defined label length, the label size will automatically be extended to contain them. This
label length extension applies only to the current label. Note that `^MNV` still requires the use of the `^LL`
command to define the initial desired label length.


- Non-continuous Media – this media has some type of physical characteristic (such as web, notch,
perforation, black mark) to separate the labels.

**Format:** `^MNa,b`



|Parameters|Details|
|---|---|
|`a =` media being used|**Values:**<br>`N =` continuous media<br>`Y =` non-continuous media web sensing1, 2<br>`W =` non-continuous media web sensing,1, 2<br>`M =` non-continuous media mark sensing<br>`A =` auto-detects the type of media during calibration1, 3<br>`V` = continuous media, variable length4<br>**Default:** a value must be entered or the command is ignored|
|`b =` black mark offset in<br>dots|This sets the expected location of the media mark relative to the point of<br>separation between documents. If set to 0, the media mark is expected to<br>be found at the point of separation. (i.e., the perforation, cut point, etc.)All<br>values are listed in dots. This parameter is ignored unless the`a` parameter<br>is set to`M`. If this parameter is missing, the default value is used.<br>**Values:**<br>`-80` to`283` for direct-thermal only printers<br>`-240` to`566` for 600 dpi printers<br>`-75` to`283` for KR403 printers<br>`-120` to`283` for all other printers<br>**Default:** 0|


1. Provides the same result.





2. This value is not supported on the KR403 printer.


3. This parameter is supported only on G-series printers.


4. This parameter is supported only on the KR403 printer.


**Comments**


This command is ignored on the HC100™ printer.


307