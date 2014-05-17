s/^[^\/]*\/\/\~\s\?//g
t test-note
b delete

:delete; d
:test-note
h
s/\/\/\~.*//g
t print-error
b end

:print-error
s/\s*$//g
=;p;
x
s/^[^\/]*\/\/\~\s\?//g
:end
=
s/\s*$//g
