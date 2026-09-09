# P1337 — transporte mecânico de RED na sucessão de formatação

O revisor aceita RED-r1 como prova funcional para o sucessor formatado R2,
condicionado à igualdade independente de tokens Rust, literais, assertions e
ordem, exceto vírgulas finais opcionais inseridas canonicamente pelo rustfmt
em tuplas já contendo mais de um elemento; saída canônica rustfmt; freeze R2
pré-C; e GREEN sobre os bytes finais. As duas vírgulas opcionais foram vistas
no diff R1→R2 dos pares AST, sem mudança de expressão ou aridade.
Isto substitui somente a exigência logística de uma terceira compilação RED
em `p1337-review-private-fmt.md`, não sua preservação ou critérios normativos.

RED-r1 SHA-256
`90cd5edeb4ec999621b95b9db82eabe26ebd034c9c490e7b10c979ad5ee6c1c5`
compilou typst-core release e terminou testes com assertions genuínas.
Fonte antes/depois
`3ce316b5d65e2ab07b33e5d8a9fb686d865b98fd03404edc4421ed9df57c9f1f`
é estável. Não é erro de compilação, cache ou execução. Os bytes R1 ficam
preservados, e o recibo final deve nomear o transporte mecânico em vez de
afirmar que RED executou os bytes R2.

O L0, baseline, oráculos CLI, expectativas, famílias e perfil não podem mudar.
Se a prova de preservação mecânica falhar, o atalho logístico não é aplicável:
reabrir freeze e executar novo RED antes de C. Esta decisão não autoriza
formatação posterior ao candidato nem edição de testes pelo implementador.
