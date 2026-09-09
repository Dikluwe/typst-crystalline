# P1333 — revisão do candidato antes dos gates finais

Revisor `/root/p1333_review`; A/B sem atestação técnica de isolamento.
Manifesto `75eea7c2a28cd8c75098d4d8fc5b2c733c30c09b7eca52a2ce7a772a24e0e99b`.
Baseline privado conferido:
`744983aa8c41877b21c2cd55abbc5c5ccb498a7bfbbb2fc5ea8dd893567ed8b0`.
Candidato inspecionado, antes da formatação anunciada:
`6dbcb2d27370316cae13a982f44a27cf4e207bdde24a6c579813139b6ab44574`.

O delta corresponde ao contrato. `calc_abs` aplica os braços vigentes ao
primeiro elemento de items, mesmo havendo sobras. O operador `?` após o
match propaga imediatamente erro de tipo/content, overflow ou Length misto.
Somente resultado bem-sucedido alcança os mesmos guards named/aridade.
O braço vazio reproduz named antes do erro legado de aridade. Sobras nunca
produzem sucesso nem são processadas como substituto do primeiro valor.

Os corpos de cálculo e diagnóstico permaneceram iguais: checked_abs,
condição zero de Length, resultados dimensionais, Float/Decimal, nomes de
tipo e seleção do primeiro value-span posicional. Sem mutação de Args,
sem API/helper novo, sem mudança de dispatcher ou avaliação eager. O cálculo
antecipado de um valor válido é puro e seu resultado só retorna quando os
guards passam; não cria novo observável de linguagem.

A comparação extraiu `original_owner` do baseline, removeu exatamente cada
snippet histórico nele e cada sucessor/novo snippet congelado no candidato,
normalizou apenas header de linhagem e newline final, e retirou o bloco
calc_abs dos dois textos. Os restantes bytes são iguais. Isso inclui
registro de nomes, outras funções, imports e testes anteriores fora dos
snippets. O script reproduzível `p1333-review-candidate-audit.cjs` registra
hash atual e também confere todos os artefatos do freeze sem escrever inputs.

Veredito de implementação: nenhuma objeção substantiva no delta inspecionado.
Ressalva de etapa: formatação/resselo e GREEN ainda pendentes no instante
desta inspeção; os gates e esta auditoria devem identificar o estado final.
O parecer não substitui execuções finais nem fecha os guards adiados P1334.
