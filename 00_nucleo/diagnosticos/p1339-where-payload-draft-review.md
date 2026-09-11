# P1339 — revisão da minuta de payload unit

Revisão de projeto, sem implementação, em 2026-09-10 aproximadamente
00:27–00:30 UTC. Regime executado sem atestação de isolamento. HEAD produtivo
`2f42d64253547734564513a1159ee6b584c1c4b4`; nenhum diff produtivo durante as
leituras. A forma pública em análise adiciona uma variante unit
`ElementPayload::NativeElement`, sem Kind adicional.

Entrada revisada: `00_nucleo/prompts/entities/element_payload.md`, SHA-256
`c5adb9bac9ffd0b550c8583a8bb29620b2a95a2e07c8ab05affa6eb92b331776`.
O hash identifica esta minuta, não aprovação humana nem selo de implementação.
Leitura integral do antecedente está registrada na revisão R2; neste exame
foram lidos o novo cabeçalho e a seção P1339 completa. Não houve edição de L0.

Proveniência do estado documental: o diff/stat observado era o de R2 mais
`compiler/stdlib/foundations/selector.md` (35 linhas adicionadas) e
`entities/element_payload.md` (77 linhas adicionadas), totalizando 11 arquivos
e 602 inserções. Caminhos completos dos demais arquivos estão no diff/stat
de `p1339-where-query-design-review-r2.md`. As contagens não são métricas de
qualidade; registram a árvore não commitada em que a minuta foi examinada.

## Evidência anterior ao parecer

1. `compiler/introspect.rs:1278-1363` liga Some payload à alocação de Location,
   gravação canônica de Content/snapshot e emissão Start. `:1944-1948` emite
   End com `hash_content(content)` para essa mesma Location.
2. `entities/content_hash.rs:25-29` inclui o Debug estrutural do Content inteiro.
   `compiler/introspect/convergence.rs:29-32` hasheia todo o slice de Tags,
   incluindo End. Portanto dois Starts unit iguais não bastam para concluir
   que mudanças de body sejam invisíveis à convergência.
3. `entities/elements/strong.rs:25-26` e `emph.rs:22-23` guardam body; mudanças
   nele chegam ao hash do Content. O discriminante Strong/Emph também chega
   ao Debug (`entities/content.rs:1207-1208`). Hash não é o matcher linguístico.
4. `compiler/eval/selector_matching.rs:283-290` reconhece origem semântica
   de Styled por flags de delta. Os testes são independentes: um único
   Styled com ambos os flags pode satisfazer os dois predicados. A forma
   pública de Styles permite tal combinação (`entities/style.rs:192-198`).
5. `Content::strong` e `Content::emph` hoje produzem variantes próprias
   (`entities/content.rs:1443-1450`). O predicado de show também aceita outras
   formas, mas não define sua quantidade de ocorrências locatáveis. A
   existência de wrapper Styled e de filho Strong não demonstra por si só
   se representam duas ocorrências ou dois carriers da mesma ocorrência.
6. O walk passa a chain ao descendente (`introspect.rs:1896-1898`), enquanto
   End hasheia o Content recebido. Se um campo público for derivado apenas
   da chain ancestral e não estiver nesse Content, esse End, isoladamente,
   não evidencia mudança do campo. Não há prova nesta revisão de que tal
   caso seja necessário nos filtros de body medidos; a possibilidade impede
   apenas uma afirmação de cobertura geral de campos derivados.

## Parecer

A forma unit não precisa de `content_hash` adicional para detectar mudança
de body representada no Content: essa informação já está em End. Acrescentar
o mesmo hash no Start duplicaria informação e não corrigiria eventual campo
derivado exclusivamente da chain. A variante proposta conserva o caráter
fechado do protocolo se os produtores forem delimitados, e evita o conflito
do Kind genérico com a proibição de catch-all.

A minuta distingue corretamente mudança pública adicional, aprovação ainda
pendente e dependência de owners antes da implementação. É adequada como
**proposta de carrier**, sem constituir desenho completo dos consumers nem
PASS funcional.

Há uma precisão necessária antes de tratá-la como autorização de produtores:
`element_payload.md:328-329` inclui “suas origens semânticas equivalentes”, mas
a equivalência no matcher de show não estabelece a unidade de ocorrência.
O L0 proprietário deve exigir medição de origem, quantidade, ordem,
ancestralidade e não duplicação antes de converter um Styled em evento.
Não pode promover automaticamente cada flag de render ou de origem para
uma nova Location, nem colapsar Strong e Emph semanticamente distintos em
uma única ocorrência porque os flags caibam no mesmo Styled.

Até essa prova, os produtores diretamente suportados pela inspeção são
Content::Strong e Content::Emph próprios. Isso não rejeita o carrier e não
exige outro campo público: delimita a autoridade operacional da expressão
“origens equivalentes”. Recomenda-se tornar explícita na minuta essa
dependência de medição/owner, sem introduzir normalização ou fase nova
incidentalmente para resolver os casos opacos.

Também se recomenda explicitar que a inferência de suficiência de End vale
para os dados representados no nó. Perda de campos derivados da chain deve
reabrir a projeção/convergência no owner correspondente; não inventar default,
não hashear filtro do usuário como se fosse dado da ocorrência e não duplicar
hash_content do mesmo nó como reparo aparente.

Resultado: **parecer favorável à forma pública unit, condicionado à precisão
de scope das origens Styled e da evidência de campos derivados**. Não é
aprovação humana, autorização de código, selo ou atestação de isolamento.
