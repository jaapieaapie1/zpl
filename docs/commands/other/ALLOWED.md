# Allowed Characters in File Names



Files on the internal printer drives (R:, E:, etc.) can be created or accessed using several different
commands (for example, ^DF, ^XF, ^TO, etc.). The names of the file can contain ONLY the characters
shown here


Shaded areas indicate characters which cannot be used. The command and control characters (normally ^
and ~) cannot be used unless the control characters are changed to something else using the ^CC~CC ZPL
command.


58


ZPL Commands

|Char.|DEC|OCT|HEX|Col5|Char.|DEC|OCT|HEX|Col10|Char.|DEC|OCT|HEX|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|(sp)<br>!<br>"<br>#<br>$<br>%<br>&<br>'<br>(<br>)<br>*<br>+<br>,<br>-<br>.<br>/<br>0<br>1<br>2<br>3<br>4<br>5<br>6<br>7<br>8<br>9<br>:<br>;<br><<br>=<br>><br>?|32<br>33<br>34<br>35<br>36<br>37<br>38<br>39<br>40<br>41<br>42<br>43<br>44<br>45<br>46<br>47<br>48<br>49<br>50<br>51<br>52<br>53<br>54<br>55<br>56<br>57<br>58<br>59<br>60<br>61<br>62<br>63|0040<br>0041<br>0042<br>0043<br>0044<br>0045<br>0046<br>0047<br>0050<br>0051<br>0052<br>0053<br>0054<br>0055<br>0056<br>0057<br>0060<br>0061<br>0062<br>0063<br>0064<br>0065<br>0066<br>0067<br>0070<br>0071<br>0072<br>0073<br>0074<br>0075<br>0076<br>0077|0x20<br>0x21<br>0x22<br>0x23<br>0x24<br>0x25<br>0x26<br>0x27<br>0x28<br>0x29<br>0x2a<br>0x2b<br>0x2c<br>0x2d<br>0x2e<br>0x2f<br>0x30<br>0x31<br>0x32<br>0x33<br>0x34<br>0x35<br>0x36<br>0x37<br>0x38<br>0x39<br>0x3a<br>0x3b<br>0x3c<br>0x3d<br>0x3e<br>0x3f||@<br>A<br>B<br>C<br>D<br>E<br>F<br>G<br>H<br>I<br>J<br>K<br>L<br>M<br>N<br>O<br>P<br>Q<br>R<br>S<br>T<br>U<br>V<br>W<br>X<br>Y<br>Z<br>[<br>\<br>]<br>^<br>_|64<br>65<br>66<br>67<br>68<br>69<br>70<br>71<br>72<br>73<br>74<br>75<br>76<br>77<br>78<br>79<br>80<br>81<br>82<br>83<br>84<br>85<br>86<br>87<br>88<br>89<br>90<br>91<br>92<br>93<br>94<br>95|0100<br>0101<br>0102<br>0103<br>0104<br>0105<br>0106<br>0107<br>0110<br>0111<br>0112<br>0113<br>0114<br>0115<br>0116<br>0117<br>0120<br>0121<br>0122<br>0123<br>0124<br>0125<br>0126<br>0127<br>0130<br>0131<br>0132<br>0133<br>0134<br>0135<br>0136<br>0137|0x40<br>0x41<br>0x42<br>0x43<br>0x44<br>0x45<br>0x46<br>0x47<br>0x48<br>0x49<br>0x4a<br>0x4b<br>0x4c<br>0x4d<br>0x4e<br>0x4f<br>0x50<br>0x51<br>0x52<br>0x53<br>0x54<br>0x55<br>0x56<br>0x57<br>0x58<br>0x59<br>0x5a<br>0x5b<br>0x5c<br>0x5d<br>0x5e<br>0x5f||`<br>a<br>b<br>c<br>d<br>e<br>f<br>g<br>h<br>i<br>j<br>k<br>l<br>m<br>n<br>o<br>p<br>q<br>r<br>s<br>t<br>u<br>v<br>w<br>x<br>y<br>z<br>{<br>}<br>~<br>(del)|96<br>97<br>98<br>99<br>100<br>101<br>102<br>103<br>104<br>105<br>106<br>107<br>108<br>109<br>110<br>111<br>112<br>113<br>114<br>115<br>116<br>117<br>118<br>119<br>120<br>121<br>122<br>123<br>124<br>125<br>126<br>127|0140<br>0141<br>0142<br>0143<br>0144<br>0145<br>0146<br>0147<br>0150<br>0151<br>0152<br>0153<br>0154<br>0155<br>0156<br>0157<br>0160<br>0161<br>0162<br>0163<br>0164<br>0165<br>0166<br>0167<br>0170<br>0171<br>0172<br>0173<br>0174<br>0175<br>0176<br>0177|0x60<br>0x61<br>0x62<br>0x63<br>0x64<br>0x65<br>0x66<br>0x67<br>0x68<br>0x69<br>0x6a<br>0x6b<br>0x6c<br>0x6d<br>0x6e<br>0x6f<br>0x70<br>0x71<br>0x72<br>0x73<br>0x74<br>0x75<br>0x76<br>0x77<br>0x78<br>0x79<br>0x7a<br>0x7b<br>0x7c<br>0x7d<br>0x7e<br>0x7f|



59