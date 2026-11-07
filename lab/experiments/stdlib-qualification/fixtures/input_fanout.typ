#import "input_modules/m00.typ" as m00
#import "input_modules/m01.typ" as m01
#import "input_modules/m02.typ" as m02
#import "input_modules/m03.typ" as m03
#import "input_modules/m04.typ" as m04
#import "input_modules/m05.typ" as m05
#import "input_modules/m06.typ" as m06
#import "input_modules/m07.typ" as m07
#import "input_modules/m08.typ" as m08
#import "input_modules/m09.typ" as m09
#import "input_modules/m10.typ" as m10
#import "input_modules/m11.typ" as m11
#import "input_modules/m12.typ" as m12
#import "input_modules/m13.typ" as m13
#import "input_modules/m14.typ" as m14
#import "input_modules/m15.typ" as m15

#let qualification_ok = true
#let qualification_inputs = sys.inputs
#assert(m00.values == sys.inputs)
#assert(m15.values == sys.inputs)
Input fanout #sys.inputs.len() pairs
Probe #sys.inputs.at("probe", default: "none")
