# ^CO




ZPL Commands


The `^CO` command is used to change the size of the character cache. By definition, a **character cache**
(referred to as cache) is a portion of the DRAM reserved for storing scalable characters. All printers have a
default 40K cache that is always turned on. The maximum single character size that can be stored, without
changing the size of the cache, is 450 dots by 450 dots.


**Cache On**


There are two types of fonts used in Zebra printers: bitmapped and scalable. Letters, numbers, and
symbols in a bitmapped font have a fixed size (for example: 10 points, 12 points, 14 points). By comparison,
scalable fonts are not fixed in size.


Because their size is fixed, bitmapped fonts can be moved quickly to the label. In contrast, scalable fonts
are much slower because each character is built on an as-needed basis before it is moved to the label. By
storing scaled characters in a cache, they can be recalled at a much faster speed.


The number of characters that can be stored in the cache depends on two factors: the size of the cache
(memory) and the size of the character (in points) being saved. The larger the point size, the more space in
the cache it uses. The default cache stores every scalable character that is requested for use on a label. If
the same character, with the same rotation and size is used again, it is quickly retrieved from cache.


It is possible that after a while the print cache could become full. Once the cache is full, space for new
characters is obtained by eliminating an existing character from the print cache. Existing characters are
eliminated by determining how often they have been used. This is done automatically. For example, a 28point **Q** that was used only once would be a good candidate for elimination from the cache.


Maximum size of a single print cache character is 1500 dots by 1500 dots. This would require a cache
of 274K. When the cache is too small for the desired style, smaller characters might appear but larger
characters do not. If possible, increase the size of the cache.

**Format:** `^COa,b,c`







|Parameters|Details|
|---|---|
|`a =` cache on|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`b =` amount of additional<br>memory to be added to<br>cache (in K)|**Values:** `1` to`9999`<br>**Default:** `40`|
|`c =` cache type|**Values:**<br>`0 =` cache buffer (normal fonts)<br>`1 =` internal buffer (recommended for Asian fonts)<br>**Default:** `0`|


**Example:** To resize the print cache to 62K, assuming a 22K existing cache:
```
^COY,40
```

**Example:** To resize the print cache to 100K, assuming a 22K existing cache:
```
^COY,78

```

163


ZPL Commands


**Print Cache Performance**

For printing large characters, memory added to the cache by the `^CO` command is not physically added to
the 22K cache already in the printer. In the second example above, the resulting 100K cache is actually two
separate blocks of memory, 22K and 78K.


Because large characters need contiguous blocks of memory, a character requiring a cache of 90K would
not be completely stored because neither portion of the 100K cache is big enough. Therefore, if large
characters are needed, the `^CO` command should reflect the actual size of the cache you need.

Increasing the size of the cache improves the performance in printing scalable fonts. However, the
performance decreases if the size of the cache becomes large and contains too many characters. The
performance gained is lost because of the time involved searching the cache for each character.


**Comments:** The cache can be resized as often as needed. Any characters in the cache when it is resized
are lost. Memory used for the cache reduces the space available for label bitmaps, graphic, and fonts.


Some Asian fonts require an internal working buffer that is much larger than the normal cache. Since most
fonts do not require this larger buffer, it is now a selectable configuration option. Printing with the Asian
fonts greatly reduces the printer memory available for labels, graphics, fonts, formats, and label bitmaps.


**NOTE:** If you have firmware x.12 or greater this command is not required because the printer
firmware automatically expands the size of the character cache as needed.


164