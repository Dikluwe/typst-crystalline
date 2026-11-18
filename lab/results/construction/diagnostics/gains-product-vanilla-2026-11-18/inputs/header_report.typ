#set page(width:400pt,height:600pt)
#set heading(numbering:"1.")
#set page(header:context [Report — #counter(page).display()],footer:context [Page #counter(page).display()])
= Header report
#for i in range(100) [Paragraph #i with several words across lines.

]
