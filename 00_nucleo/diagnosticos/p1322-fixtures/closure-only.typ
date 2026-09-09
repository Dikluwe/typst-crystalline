#import "sub/module.typ": load
#assert.eq(load(), (("local", "value"), ("sub", "42")))
Relative string in closure.
