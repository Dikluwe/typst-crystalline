#set heading(numbering: "1.")
#set math.equation(numbering: "(1)")

#set text(lang: "fr")
Forward: @de-heading.

#set text(lang: "de")
= Deutsch <de-heading>
$ x = 1 $ <gleichung>

#set text(lang: "es")
= Español <es-heading>

#set text(lang: "fr")
= Français <francais>

#set text(lang: "it")
= Italiano <it-heading>

#set text(lang: "fr")
Deutsch: @de-heading. Gleichung: @gleichung.

#set text(lang: "de")
Français: @francais.

#set text(lang: "it")
Español: @es-heading.

#set text(lang: "es")
Italiano: @it-heading.

#set text(lang: "en")
Explicit: @de-heading[Abs.].
Empty: @de-heading[].
