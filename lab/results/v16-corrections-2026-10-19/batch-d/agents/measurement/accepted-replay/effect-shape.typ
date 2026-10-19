#set page(width:200pt,height:150pt,margin:0pt)
#let tally=state("effect-shape",0)
#grid(columns:(auto,10pt),gutter:0pt,[#tally.update(n=>n+1)#rect(width:40pt,height:20pt,fill:green,stroke:none)],rect(width:10pt,height:10pt,fill:rgb("ff0000"),stroke:none))
#context assert(tally.final()==1,message:"effect repeated")
