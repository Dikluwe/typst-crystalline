#set page(width:120pt,height:100pt,margin:10pt)
#set text(size:19pt)
#counter("x").update(7)
$cancel(x, angle: #(a => { let v = counter("x").get(); 0deg }))$
