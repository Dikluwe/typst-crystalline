// The regex substitutions make whitespace semantics visible as ASCII in HTML.
// A newline between CJK ideographs disappears; an explicit space remains.
#show regex("水果"): [CJK_JOINED]
#show regex("水 果"): [CJK_EXPLICIT_SPACE]

水
果

水 果
