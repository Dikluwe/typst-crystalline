#show heading.where(level:2):it=>it.body
#show heading.where(level:1):it=>{show par:it=>{metadata(repr(it));it};[A#heading(level:2)[B]C]}
#heading[X]