#set page(width:200pt,height:150pt,margin:0pt)
#let tally=state("effect-equation",0)
#grid(columns:(auto,10pt),gutter:0pt,[#tally.update(n=>n+1)$x + y$],rect(width:10pt,height:10pt,fill:rgb("ff0000"),stroke:none))
#context assert(tally.final()==1,message:"effect repeated")
