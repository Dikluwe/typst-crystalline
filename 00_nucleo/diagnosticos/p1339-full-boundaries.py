"""Additional source-informed public boundaries; isolated from product/L0."""
import importlib.util
from pathlib import Path
import sys

spec=importlib.util.spec_from_file_location('probe',Path(__file__).with_name('p1339-full-probe.py'))
m=importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

def cases():
    result=[]
    def add(id,route,expression,obligation):
        result.append(dict(id=id,route=route,expression=expression,obligation=obligation,kind='source-informed-exploratory',preclassification='Unknown',preclassification_basis='public boundary discovered before any L0 or candidate; no result presumed'))
    for unit in ['deg','rad']:
        for name,expr in [('nan-multiply','float("nan") * 1'+unit),('infinite','float("inf") * 1'+unit),('zero-infinite','0'+unit+' * float("inf")'),('inf-minus-inf','float("inf") * 1'+unit+' - float("inf") * 1'+unit)]:
            add('angle-construction-'+unit+'-'+name,'angle.'+unit,'{ let a = '+expr+'; repr((type(a),a,a==0deg,a/1deg,(a/1deg)==(a/1deg))) }','bilateral construction control: distinguish real NaN Angle from normalized zero')
    for val in ['0.1','-0.1','1e-46','1e-45','1e40','-1e40']:
        for endian in ['little','big']:
            add('bytes-rounding-'+val+'-'+endian,'float.to-bytes','{ let b = float.to-bytes('+val+',size:4,endian:"'+endian+'"); let x = float.from-bytes(b,endian:"'+endian+'"); repr((array(b),x)) }','binary32 rounding,subnormal,underflow,overflow')
    for args in ['1.0,size:4294967296','1.0,size:true','1.0,endian:none']:
        add('bytes-cast-'+args,'float.to-bytes','repr(float.to-bytes('+args+'))','size/endian cast boundaries')
    for form in ['static','bound']:
        for args in ['level:"bad"','level:panic("field"),not-a-field:1','not-a-field:panic("field"),level:1','not-a-field:1,level:panic("later")']:
            call='function.where(heading,'+args+')' if form=='static' else 'heading.where('+args+')'
            add('where-field-priority-'+form+'-'+args,'function.where','repr('+call+')','where field values are not cast; argument evaluation precedes field-name validation')
    for call in ['version(1).at()','version(1).at(index:0)','version(1).at("0")','version.at(self:version(1),0)','version.at(version(1),index:0,other:1)']:
        add('version-remaining-'+call,'version.at','repr('+call+')','bound missing,type,named and static self positional diagnostics')
    for route,args in [('angle.rad','self:90deg'),('float.from-bytes','bytes:bytes((0,0,192,63))'),('function.with','self:calc.abs'),('function.where','self:heading')]:
        add('named-required-'+route,route,'repr('+route+'('+args+'))','declared positional parameter provided by name')
    for args in ['1,0,other:panic("arg")','version(1),panic("index"),other:panic("other")']:
        add('version-eval-order-'+args,'version.at','repr(version.at('+args+'))','index/receiver casts happen after argument evaluation')
    return result

m.cases=cases
m.main()
