# P1292 — amendment-9: retomada de fluxo e default de `place.clearance`

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
classificação inicial
`0f3a2e91daf9f6c83c0c488c6aff12b3fc479e2a089e479842a42252653e351d`;
retomada após gate
`48545a4d9ba62a148f9161aca43dbfd93a6618c1816e480d245a1ad554d2e243`.

**Gate ADR-0127:** confirmado pelo humano com `Autorizo` em
`2026-09-01T08:46:11-03:00`, para mudar a omissão de `clearance` para
`1.5em` e concluir o mecanismo interno medido.

**Estado final:** contrato owner-correct resselado; entrega causal ao
implementador, sem código/oráculo/teste/ataque/veredito por este papel.

## Proveniência da medição

- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`;
- working tree não commitada, medida em `2026-09-01T03:24:35-03:00`;
- `git diff HEAD --stat` antes desta minuta: 41 arquivos, 2.055 inserções e
  351 remoções;
- retomada pós-gate medida em `2026-09-01T08:50:07-03:00`; antes do resselo,
  `git diff HEAD --stat` registrou 42 arquivos, 2.192 inserções e 351
  remoções;
- candidato `target/debug/typst` SHA-256
  `70f5547047adbc261a69d7e7b77b0df1c0c96c69e85bbb47e6a5afa913947d1d`;
- vanilla ratificado `/usr/local/bin/typst` SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Os números abaixo vêm de PDFs compilados bilateralmente e lidos por
`pdfinfo` e `pdftotext -bbox`. As fixtures independentes estão em
`/tmp/p1292-a9-current/`; os seus hashes constam ao fim deste documento.

## Reprodução da refutação v9

| Caso | Vanilla ratificado | Candidato v9 | Resultado |
|---|---|---|---|
| prefix/suffix, default omitido | 3 páginas: PREFLOW p1; FLOATPREFIX p2 `y=90.166`; AFTER + FLOATSUFFIX p3 | 3 páginas, mas AFTER e FLOATPREFIX coexistem em p2; token do float `y=10.166`, suffix p3 | ordem causal e posição refutadas |
| top+bottom, default omitido | 2 páginas: TOP + flow p1; AFTER + BOTTOM p2 | 1 página, quatro tokens sobrepostos | contagem e posição refutadas |
| boundary, `clearance: 5pt` explícito | 2 páginas; AFTER p2 `y=-2.596`, float p2 `y=90.166` | 2 páginas; AFTER p2 `y=-2.596`, float p2 `y=65.166` | contagem igual não implica placement bilateral |

Isto reproduz exatamente os três refutadores que abriram o amendment-9.

## Isolação de `clearance`

No caso top+bottom, o vanilla omitido e `1.5em` foram idênticos: duas páginas,
TOP `y=-2.596` e MIXFLOW `y=33.904` em p1, AFTER `y=-2.596` e BOTTOM
`y=90.166` em p2. Com `0pt`, o vanilla ainda teve duas páginas, mas MIXFLOW
passou a `y=17.404`. Com `5pt`, MIXFLOW ficou em `y=22.404`.

No candidato, omitido e `0pt` foram idênticos e produziram uma página. Com
`1.5em`, a contagem passou a duas, mas o placement continuou divergente: TOP
`y=-9.834`, MIXFLOW `y=33.904` em p1, AFTER `y=-2.596` e BOTTOM `y=43.666`
em p2. Portanto o default explica a contagem top+bottom, mas não explica nem
corrige o alinhamento/realização dos floats.

No caso prefix/suffix, o vanilla omitido e `1.5em` foram idênticos em três
páginas. `0pt` reduziu o vanilla para duas páginas. No candidato, omitido e
`0pt` foram idênticos em três páginas com AFTER e FLOATPREFIX em p2; com
`1.5em`, a contagem continuou três e a sobreposição causal permaneceu.

A fonte ratificada `crates/typst-library/src/layout/place.rs:139-144`
(SHA-256
`ec64092c3f09428aa5d5c2e43a35849cfa2e062947d8d662066a4cdfc3541579`)
declara `#[default(Em::new(1.5).into())]`. `flow/collect.rs:316-324` resolve o
valor no child colocado; `flow/compose.rs:341-348` inclui o clearance no
fitting somente quando há conteúdo; e `flow/compose.rs:748-785` o aplica na
reserva/posição de top e bottom. Estes passos são mecânica upstream; o
observável de linguagem é a equivalência da omissão com `1.5em`.

**Classificação:** a hipótese “default zero é suficiente e a falha é apenas
do marker” está refutada. Mudar a omissão do candidato de zero para `1.5em`
é mudança de comportamento por defeito; o gate ADR-0127 foi confirmado.

### Unidade relativa confirmada após o gate

Controles ratificados com top float de 20pt compararam omissão,
`clearance: 1.5em` e `clearance: 0pt` em dois estilos. A 10pt, omitido e
`1.5em` colocaram FLOW em `y=32.640`, contra `17.640` para zero: delta de
15pt. A 20pt, os pares ficaram em `y=45.280`, contra `15.280`: delta de 30pt.
Logo `1.5em` é preservado como unidade relativa e resolvido contra o estilo
efetivo; assar os 16.5pt do estilo default de 11pt está refutado.

## Auditoria de retomada e ownership

A mesma fixture prefix/suffix foi repetida substituindo o flow posterior por
texto simples e por `block(height: 10pt, breakable: false)`. Nos dois casos o
candidato manteve o posterior em p2 junto do prefixo; o vanilla o iniciou em
p3. Assim, um re-preflight especial apenas em `Block` não é solução geral e
fica refutado.

No upstream ratificado, `flow/distribute.rs:117-153` processa cada child na
ordem; `:327-391` decide fitting de single/multi no estado de região já
composto; e `:514-521` faz a sentinela terminar a região enquanto há floats
pendentes. `flow/compose.rs:82-106` e `:200-234` repetem a composição quando
uma inserção float exige relayout e só então finalizam/reservam as inserções.
A obrigação de linguagem é que a região corrente já reflita os floats
realizados antes de o child seguinte ser processado. `Stop`, checkpoints e
relayout são evidência mecânica, não API a copiar.

Ownership medido:

- `compiler/layout/flush.md`: somente política/fronteira da sentinela;
- `compiler/layout/cursor.md`: avanço e coordenação da região paginada;
- `compiler/layout/place.md`: fitting, reserva, alinhamento e emissão float;
- `compiler/layout/block.md`: fitting do próprio block, sem conhecer marker;
- `compiler/stdlib/layout.md`: extração e default público de `clearance`.

## Decisão mínima após a confirmação

`compiler/stdlib/layout.md` torna vigente a omissão como `Length` relativo
`1.5em`, resolvido no layout; valores explícitos permanecem soberanos.

`compiler/layout/place.md` possui fitting, reserva, coordenadas top/bottom e
o pedido interno de recomposição quando uma inserção muda a região. Fitting
inclui clearance se já há flow; região vazia pode admitir o próprio float sem
clearance, mas a reserva realizada sempre separa o futuro flow pelo clearance.
O frame bottom ancora no fundo da região, não no cursor usado.

`compiler/layout/cursor.md` possui checkpoint, replay idempotente por
ocorrência, estabilização e avanço. Depois de Place alterar reservas, recompõe
a região contra sua área efetiva; só consome Flush e retoma o child seguinte
quando o prefixo não está pendente e a região está estável. Nunca pergunta o
tipo/altura do próximo child. Texto e Block decidem fitting normalmente.

`compiler/layout/flush.md` conserva apenas fronteira e política: chama o hook
e exige retorno sobre região estabilizada. Não contém fitting, distribuição,
clearance, frame, replay ou quebra incondicional.

Isto explica bilateralmente os controles: com default `1.5em`, o bottom float
80pt é admitido numa página vazia, mas reserva 96.5pt e o AFTER de 10pt progride
para p3; com `0pt`, reserva 80pt e o AFTER cabe em p2. Top+bottom e boundary
usam as mesmas reservas/coordenadas e devem igualar posição, não apenas páginas.

`compiler/layout/block.md` foi auditado e permaneceu inalterado: a prova com
texto simples refuta dependência de Block. Não há nova fase, passagem, API,
item, segundo distribuidor ou inspeção do conteúdo posterior.

## L0s do amendment-9

```text
compiler/stdlib/layout.md
fd12e67bf884c36ed249ad8c20ce250bb129726baa0d4c9244ecedf6aff54dfa

compiler/layout/place.md
51b9fbd113f919bc7d3008b6abca4590eb985b1d91ae51075833ac0cd8ae4f97

compiler/layout/cursor.md
9ea8fa7215e1f44e447f13e2bea08f31aa12379450dcee2f734833d4c1c85f3c

compiler/layout/flush.md
c105f59a3e6f3013ba76db514ec982c25826c8cd64f2b0e8b13a04815899ca1a

compiler/layout/block.md (auditado, inalterado)
55eb249c62c264786b28aec6351515aa889e1d010241f3481e9dbc59b617277f
```

O owner Place entra no grafo canônico; o set passa de 24 para 25 L0s. Os
`Hash do Código` não foram artificialmente resselados e `--fix-hashes` não
foi usado.

## Hashes das fixtures adicionais

```text
top-bottom-clearance-0.typ       96d2e3a2cc2ff361fc169a58c8634f6e15c607b75e2babf133be086a29dc5499
top-bottom-clearance-5.typ       dd657ae24229dd0691e17daf6649342b3b0cf3dc7bc4e56781d7a69577997eb6
top-bottom-clearance-1_5em.typ   a33e20bad01fa6ed5f77135d28b3f27fb5816f6efd83168be1af955e71948d78
prefix-clearance-0.typ           04f0015f0e84905ab06a71fb3e78d830e276785ede8a034a5314c04a8b87deb4
prefix-clearance-5.typ           7b552df041f6edbc6ca7c380556d394dfba052da1ec76eb6e46a6434ff474dc5
prefix-clearance-1_5em.typ       49d078154566c79773d91e589a5dc1a2a4f6609b614be075861914599a595717
prefix-after-breakable-false.typ 8d32c94971e1378c3e3dfb394d9872775875cc452cc0a5fed440c8623779e180
prefix-after-text.typ            056465d906c831d0d4200472e26cba2440c2a98c385342fd408c1052a21ca11a
relative-default-10-omitted.typ  f4bad6c41579128849a3f2ff4d432becf09d117d5ec8b95c2d51c09e1929bfbe
relative-default-10-explicit.typ eb511e63dc1d656d779a6aaf5de364cc0906dee8f33fef7762a14c9ebf3bad05
relative-default-10-zero.typ     036991a1c5aa9939ab7e18a87939ece4f7853a2b5f69c1a8e7b50e110ce0734e
relative-default-20-omitted.typ  0a4ad349a53ada38357feca6c6459d021754b24afa90cf0019e06dfbff2c818d
relative-default-20-explicit.typ 3172400e0a70495fdda78e91b7b4cd74ef819658cd6f15f4ab3a6b257c6ace96
relative-default-20-zero.typ     1ee251569f8304270059b2a67d6b3ef35dae6830714cda1ffb864f24824fbf88
```

Contrato/receipt/gates finais são registrados após o resselo. **PARAGEM.**
