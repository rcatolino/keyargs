s/\/\/\~//g
T move-on
d

:move-on
/error:\|note:/s/^[^0-9]*//g
t print-line-nb
d

:print-line-nb
s/:.*\(error:\|note:\)/\n\1/g
T abort
b

:abort
d
