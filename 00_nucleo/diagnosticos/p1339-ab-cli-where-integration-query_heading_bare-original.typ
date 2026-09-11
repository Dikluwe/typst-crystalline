#set heading(numbering: "1.1")
= First
== Child
= Last
*Bold* _Emph_ Text
#context metadata(query(heading).map(it => (it.level, repr(it.body))))
