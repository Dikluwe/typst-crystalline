# V-P1284-v1 — certificado de verificação independente

**Estado:** `FINAL_VERDICT`  
**Veredito:** **Conformant**  
**Contrato aplicável:** `C-P1284-v7`  
**Oráculo aplicável:** `A-P1284-v5`  
**Ataque aplicável:** `ADV-P1284-v2`  
**Instante final da medição:** `2026-08-30T12:48:23-03:00`

## 1. Declaração

O fragmento P1284 satisfaz o contrato final C-P1284-v7. Não há `Unknown`
contabilizado como sucesso e não há blocker técnico aberto dentro do escopo
autorizado. As superfícies reservadas ao gate ADR-0127 continuam ausentes ou
bloqueadas, sem stub e sem crédito. Os quatro residuais de query/serialização,
CLI e introspecção estão corretamente separados para P1285 e não foram
convertidos em evidência semântica de P1284.

Este papel não escreveu C-P1284-v7, A-P1284-v5, a candidata, os testes nem
ADV-P1284-v2. A única alteração feita por este papel é o presente certificado.

## 2. Identidades verificadas

| Artefato | SHA-256 recalculado |
|---|---|
| `p1284-contract-receipt.md` — C-P1284-v7 | `ea85130ae2954d0fb930e68ce82b5554780798ef5927bcab0cc7f109ab9c61f6` |
| `p1284-oracle-receipt.md` — A-P1284-v5 | `1c04efc1c2100ecc47b7891cb7962beaca64895fc769c3ca509efc084196519d` |
| `p1284-probes.json` — suíte A-P1284-v5 | `e8c6a0a2da6dbb784e29ba65e7cfa41b5eb17877e1f1f7fe5760e37d09f89d8f` |
| `p1284-candidate-oracle-receipt.json` | `d14ae6e871f5bb2efe421208774f00577d3142a7c09e1314c288535ee0fce431` |
| `p1284-adversarial-receipt.md` — ADV-P1284-v2 | `3c8064546ab52c15f793583ab98246362f14098bf3bd3a7f4f398ce404d8141f` |
| `p1284-summary.json` | `19f460ec8445b8869bf22f8354661d77c33e1ea0ffcb065a561dbc5f130a35f2` |
| inventário default / HTML | `41a16a2856b335275656af64649f9bbd85465ff9ebf91c979e3398ae3650db4e` / `2a3f5f7d19497f702e8a410c1a0a94a793e5a4420d4a98203ef2268bbabf37f3` |
| probes default / HTML | `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb` / `828284bcb3f1676c52c9e32944506e4153656db7898b7b06ad07569463386db1` |
| `p1284-residual-p1285.json` | `4da83da19c0f97467286814faac79e6faacbfd46e3cf7436a226008ec73eb6fd` |
| `typst-passo-1284-relatorio.md` | `d17e97b70b02d7bb411eecffad55d54fd6ca67974e9b54a0d22b0561f394700f` |
| candidata `target/release/typst` | `8b85f933b7cd1fa74e46e2c18902b8343d9064f11b2a76844a476a252835a57e` |
| vanilla `/usr/local/bin/typst` | `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8` |

Os pins cruzados de contrato, oráculo, suíte e recibo candidato em ADV-P1284-v2
coincidem com estes hashes. A candidata manteve a identidade exigida antes e
depois de `cargo build --release`.

## 3. Causalidade L0 e ownership

O primeiro ensaio deste verificador detectou corretamente que C-P1284-v6 e
A-P1284-v4 pinavam sete hashes L0 anteriores ao resselo do `Hash do Código`;
nesse estado o veredito seria bloqueado. A cadeia foi substituída, sem alterar
a candidata, por C-P1284-v7/A-P1284-v5/ADV-P1284-v2.

No estado final:

- os 23/23 L0 pinados por C-v7 e A-v5 coincidem integralmente com os ficheiros
  atuais;
- cada um dos 23 L0 possui exatamente um consumer produtivo encontrado por
  seu header `@prompt`, sem colisão 1:N;
- para cada um dos sete L0 resselados, a substituição **em memória** apenas da
  linha atual `Hash do Código` pelo valor C-v6 reproduziu exatamente o SHA-256
  integral C-v6; isto refuta alteração normativa adicional;
- a projeção semântica A-v5 declara o mesmo hash canônico anterior,
  `4159f8e5b0f683b85eac992f0a99c04e9241d14294721780927fbd4790f3c16c`;
- `crystalline-lint . --checks v15,v26` terminou com `No violations found`.

Assim, o resselo tardio é aceito somente como repin mecânico comprovado e não
como adaptação retroativa das expectativas ao candidato.

## 4. Oráculo focal e inventário

O runner `run_p1284_oracles.py` foi reexecutado independentemente contra o
binário congelado e a suíte A-v5. Sua saída temporária foi byte a byte idêntica
ao recibo candidato final, inclusive SHA-256 `d14ae6e8...e431`:

- `36/36` casos funcionais/diagnósticos;
- `1/1` controle aninhado: `array.push(x, 3)` falha ao tentar mutar constante;
- `15/15` mapas, com kind, cardinalidade, primeiro, último e digest integral;
- `4/4` ausências executáveis bloqueadas;
- `all_passed = true`.

O catálogo foi reconstruído mecanicamente: 154 membros, incluindo
`color.map`, mais seus 15 filhos, totalizando 169 paths únicos. Todos os
169/169 têm `active_bilateral` nos perfis default e HTML; nenhum path
autorizado está ausente ou fora dessa classificação.

Os bloqueados foram adjudicados separadamente:

- `color.spot`, `outline.entry`, `selector.before` e `selector.after` são
  `MISSING_MEMBER` nos dois perfis e falham no acesso executável;
- `color.spot.tint` e os cinco filhos de `outline.entry` são exatamente os seis
  `Unknown(blocked_by_ancestor)` do inventário, nunca sucesso;
- defaults/metadata de `outline` permanecem bloqueados; o diff produtivo de
  `structural/outline.rs` altera somente lineage;
- o residual P1285 carrega exatamente 11 entradas bloqueadas sem crédito.

## 5. Adjudicação das 35 mutações

O score foi recalculado a partir das 35 linhas individuais do contrato e das
testemunhas reproduzidas, não copiado do relatório:

| # | Resultado | Testemunha concreta independente |
|---:|---|---|
| 1 | Rejected | projeção não ligada `array.len((1,2,3))` coincide com `.len()` |
| 2 | Rejected | `type(array.len) == function` e chamada real |
| 3 | Rejected | `alignment.left` conserva kind/valor, não é função |
| 4 | Rejected | `array.range(3) == (0,1,2)` como estática |
| 5 | Rejected | formas ligada/não ligada exercitam `self` em array/direction/alignment |
| 6 | Rejected | `default:` named funciona e erros positional/named são exatos |
| 7 | Rejected | `array.at((1,2))` produz `missing argument: index` |
| 8 | Rejected | defaults de range/at/alpha são comparados exatamente |
| 9 | Rejected | `zip` cobre zero/um/múltiplos e rejeita excesso inválido |
| 10 | Rejected | named `bogus` produz `unexpected argument: bogus` |
| 11 | Rejected | len, U+0000 e dispatch são comparados por resultado, não só kind |
| 12 | Rejected | equivalência ligada/não ligada de map/inv é executada |
| 13 | Rejected | callback `all` prova curto-circuito antes de `panic` |
| 14 | Rejected | 11/11 diagnósticos/casts negativos e needles exatos |
| 15 | Rejected | valor e `repr` de direção/alinhamento são pinados |
| 16 | Rejected | aliases global/qualificado de cor/alignment/direction coincidem |
| 17 | Rejected | `type(color.map) == module` e filhos são arrays |
| 18 | Rejected | cada um dos 15 presets é acessado |
| 19 | Rejected | 15 digests integrais rejeitam ordem/tamanho/cor mutados |
| 20 | Rejected | `arguments(1,x:2).len() == 2` ligada e não ligada |
| 21 | Rejected | map/filter preservam `.named() == (x:3)` |
| 22 | Rejected | bytes/codepoints/clusters de `é` são distinguidos |
| 23 | Rejected | state/location/length cobrem presença e erro contextual |
| 24 | Rejected | `72pt.inches()==1`, 24h=1 dia e 1 semana=7 dias |
| 25 | Rejected | diff de outline contém somente atualização de lineage |
| 26 | Rejected | `pdf.attach` mantém kind e erro explícito `scope-out` |
| 27 | Rejected | seis filhos bloqueados continuam Unknown fora do score |
| 28 | Rejected | ownership 23/23 1:1; V15=0 e V26=0 |
| 29 | Rejected | before/after ausentes; and/or/within exercitados separadamente |
| 30 | Rejected | diff dos consumers não adiciona `pub`, enum, trait ou struct |
| 31 | Rejected | constructor Arguments preserva positional/named e chamabilidade |
| 32 | Rejected | CMYK cobre Ratio válido, Int/101% inválidos, ordem e alias |
| 33 | Rejected | HSL/HSV cobrem Angle, Int/Ratio, ranges e alpha default |
| 34 | Rejected | Oklab/Oklch cobrem Ratio, Angle, escala de chroma e ausência de clamp |
| 35 | Rejected | HSL/HSV/Oklab/Oklch qualificados coincidem com globais |

**Mutation score sustentado: `35/35 = 1.0`.** Nenhuma linha é `Unknown`. As
testemunhas 1–24 e 26–27/29/31–35 são observações de linguagem/recibo
reexecutado; 25/28/30 são auditorias estruturais reproduzidas.

As 10 linhas de controle também foram contadas mecanicamente e adjudicadas
`10/10 correto`, fora do numerador e denominador: ausência de spot/tint,
outline.entry e before/after; proibição de stub/alias/composição sem gate;
filhos de ancestral ausente como Unknown; 15 mapas integralmente sondados; e
and/or/within existentes e equivalentes nas duas formas.

## 6. Gates reexecutados

| Comando | Resultado observado |
|---|---|
| `python3 lab/surface-inventory/run_p1284_oracles.py ...` | `36/36`, nested true, `15/15`, 4 controles, all passed |
| `cargo test --workspace --offline --quiet` | 6.349 passed, 0 failed, 3 ignored |
| `cargo build --release -p typst-wiring --offline` | exit 0; SHA da candidata preservado |
| `python3 -m unittest discover -s lab/surface-inventory -p 'test_*.py'` | 22/22 |
| `cargo fmt --all -- --check` | exit 0 |
| `git diff --check` | exit 0 |
| `crystalline-lint . --quiet` | exit 0 |
| `crystalline-lint . --checks v15,v26` | exit 0; nenhuma violação |

Proveniência imediatamente anterior a este certificado: HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree não commitado,
117 linhas em `git status --short` (SHA-256
`9670c09f12dabcbb9bc9f55887dc62cba91ce1601fed41d13923728998cf7c18`),
e `git diff HEAD --stat` com 75 ficheiros, 644076 inserções e 1171 remoções
(SHA-256 `d87e8175fd7626c6d21cf988b926b05d1e5c3141a0cc2d2b3b756e9c665d770b`).
A única linha adicional posterior é este novo ficheiro, elevando o status para
118 linhas e SHA-256
`984e046e24cb594e30adc8ec12e88049fc96a34d38ad3eab076cfe79b6e032dc`;
o diff stat de ficheiros rastreados permanece inalterado.

## 7. Residual e limitações

`p1284-residual-p1285.json` individualiza quatro pendências: serialização de
resultado query não-heading, cobertura JSON de valores query, protocolo stdin
`query -` e metadata genérica de funções (386 default/449 HTML). Assertions de
compile provam a semântica dos 36 casos, mas **não** provam o serializer query;
esta distinção está correta e não bloqueia o contrato P1284.

Limitações do certificado:

- a árvore é compartilhada e não há atestação física de isolamento; a
  independência é procedimental, apoiada por autoria segregada e hashes;
- a árvore não commitada impede atribuir todo o estado a um commit imutável;
  por isso são registrados HEAD, status, diff stat e hash do binário;
- as 35 mutações foram adjudicadas por poder discriminatório de testemunhas
  executadas e auditorias estruturais; não foram produzidas 35 builds mutantes,
  o que teria alterado a candidata congelada e violado o regime deste passo;
- o veredito limita-se ao fragmento C-P1284-v7, não afirma paridade global do
  compilador nem fechamento dos residuais P1285.

## 8. Veredito

**Conformant.** Causalidade resselada e comprovada; ownership 1:1; 169/169
paths preservados nos dois perfis; oráculo candidato 36/36 + nested + 15/15;
mutation score 35/35 = 1.0; 10 controles fora do denominador; bloqueados sem
crédito; gates de teste, build, formato, diff e arquitetura verdes. Não há
blocker P1284 aberto no estado identificado acima.
