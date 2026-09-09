#import "sub/module.typ": captured, load
#assert.eq(csv(captured), (("local", "value"), ("sub", "42")))
#assert.eq(load(), (("local", "value"), ("sub", "42")))
Captured path and relative load.
