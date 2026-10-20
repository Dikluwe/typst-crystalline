#set page(width:120pt,height:100pt,margin:10pt)
#counter("x").update(7)
$cancel(x, angle: #(a => { counter("x").final(); 0deg }))$
