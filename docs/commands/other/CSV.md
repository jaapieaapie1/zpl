# CSV File Information



[The file format should follow the rules in IETF RFC 4180: http://tools.ietf.org/html/rfc4180](http://tools.ietf.org/html/rfc4180)


The maximum number of columns per row in a CSV file is 256.


Each row must be 2048 characters or less including the delimiter. The carriage return/line feed (CRLF)
does not count toward the limit.


Each row in the CSV file must have the same number of elements. If there are any missing elements in the
CSV file (indicated by two adjacent commas or a comma at the end of a row), they will be represented as
empty strings.


If an element in the CSV file contains a quote, it should be represented as two quotes. Additionally, if an
element contains a quote, a new line, a carriage return, or the delimiter character, the element must be
within quotes. For example, a value that is used to store a measurement in feet and inches (4' 5") must be
formatted as "4' 5""" within the CSV file.


516


ZBI Commands