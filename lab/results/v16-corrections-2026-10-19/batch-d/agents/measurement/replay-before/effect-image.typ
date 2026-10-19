#set page(width:200pt,height:150pt,margin:0pt)
#let tally=state("effect-image",0)
#grid(columns:(auto,10pt),gutter:0pt,[#tally.update(n=>n+1)#image("asset.png",width:40pt,height:20pt)],rect(width:10pt,height:10pt,fill:rgb("ff0000"),stroke:none))
#context assert(tally.final()==1,message:"effect repeated")
