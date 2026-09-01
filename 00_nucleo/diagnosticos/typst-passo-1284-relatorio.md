# Relatório do Passo 1284 — módulos, tipos e membros estáticos

**Estado:** fechado com veredito segregado **Conformant**.

## Escopo e decisão

O contrato independente `C-P1284-v7` autorizou 169 paths observáveis nas
famílias `array`, `bytes`, `arguments`, `str`, `dictionary`, `color`,
`direction`, `alignment`, `duration`, `length`, `selector`, `state`, `location`
e `pdf`. O contrato e o oráculo foram resselados antes do código quando as
medições vanilla revelaram, sucessivamente, a chamabilidade de `arguments`, os
quatro componentes Ratio de `cmyk`, os casts heterogêneos de HSL/HSV/Oklab/Oklch
e as cardinalidades integrais de `rocket`/`mako`. O v7 final preserva
integralmente a semântica v6 e repina sete L0 cujo único byte normativo alterado
foi o `Hash do Código` ressellado após a implementação; os outros 16 L0 são
byte-idênticos. A reconstrução removendo apenas esse header reproduziu os sete
hashes v6 exatamente.

Foram preservados como bloqueados, sem stub nem crédito: `color.spot` e
`color.spot.tint`; `outline.entry` e seus cinco filhos; `selector.before` e
`selector.after`; defaults/metadata públicos de `outline`. Esses casos exigem
novo gate ADR-0127. `pdf.attach` continua exposto, mas sua execução exportadora
permanece scope-out cristalino explícito.

Não houve alteração de contrato público Rust, default do produto, fase de
pipeline ou quebra de compatibilidade. As mudanças autorizadas são correções de
paridade de linguagem e tabelas, portanto seguiram o fluxo contínuo ADR-0127:
L0 atualizado primeiro, teste RED, implementação, resselo e revalidação.

## Implementação

As projeções ligadas e não ligadas foram centralizadas no dispatch estático,
delegando a semântica aos owners existentes. Coleções ganharam as superfícies e
defaults do contrato, incluindo mutação apenas sobre bindings mutáveis, callbacks
booleanos estritos, `dedup` global estável, ordenação estável, variádicos de
`zip`, callbacks de dictionary/arguments somente sobre o valor, índices Unicode
e o roundtrip U+0000. `Type::Arguments` tornou-se construtor chamável.

As superfícies de direção, alinhamento, duração, comprimento, selector, state e
location preservam seus tipos de domínio. `to-absolute`, `state.get/final/at` e
location aplicam o gate de contexto observado no vanilla. `pdf.artifact` e a
presença de `pdf.attach` foram mantidos sem alargar a exportação.

Em cor, os construtores agora respeitam os componentes exatos do contrato:
`cmyk` recebe quatro Ratio; HSL/HSV recebem Angle e componentes Int/Ratio; Oklab
e Oklch distinguem lightness Ratio, chroma numérico/Ratio e hue Angle. Os 15
filhos de `color.map` foram extraídos mecanicamente da fonte vanilla pinada e
materializados como sequências integrais, não aproximações.

## RED → GREEN e oráculo focal

O estado predecessor P1283 continha 115 membros ausentes e 44 metadados não
observados no recorte congelado de P1284. O binário predecessor tinha SHA-256
`a6e03f6c3da68e689c3baa1d06d1fd8f6665b9294786e4add145eb13b4f47917`.
A consulta direta do primeiro sentinela já era contaminada pelo serializer
restrito a headings; por isso ela não foi usada para converter falha de CLI em
falha semântica.

O oráculo final `A-P1284-v5` preserva a suíte semântica anterior e repina
`C-P1284-v7` e os 23 L0 finais. No binário final, SHA-256
`8b85f933b7cd1fa74e46e2c18902b8343d9064f11b2a76844a476a252835a57e`,
o executor reproduzível `run_p1284_oracles.py` preservou cada expressão e valor
esperado selados, substituindo apenas o nó final de metadata por uma assertion
Typst exata. Resultado:

- 36/36 itens funcionais e de diagnóstico;
- 1/1 controle aninhado: `array.push(x, 3)` não pode mutar constante;
- 15/15 mapas, por kind, cardinalidade, extremos e SHA-256 da sequência inteira;
- 4/4 controles executáveis de ausência para as superfícies bloqueadas.

O total do oráculo vanilla continua `52/52`; a aritmética é
`6 + 1 + 18 + 11 + 1 + 15`. No candidato, os 36 itens principais, o controle
aninhado e os 15 mapas passaram, com a divergência autorizada de `pdf.attach`
avaliada pelo lado cristalino.

## Limite explícito: query e serialização

P1284 não implementou nem reivindica o serializer de `typst query`. A fonte
original do sentinela S1, executada diretamente, termina com exit 1 e
`query serialization currently supports headings only`. O protocolo stdin
`typst query - ...` também ainda trata `-` como caminho literal. Portanto a
assertion compilada é evidência da semântica P1284, não evidência de query.

Esses fatos, junto dos 386/449 parâmetros de função ainda genericamente
`UNVERIFIED_METADATA` nos perfis default/HTML, estão individualizados em
`p1284-residual-p1285.json`. As contagens são observações de paths, nunca uma
percentagem de paridade.

## Inventário bilateral regenerado

O catálogo final contém 1.983 entradas cristalinas em `default` e 2.047 em
`html`. Os totais genéricos são:

| Perfil | MATCH | MISSING_MEMBER | UNVERIFIED_METADATA | EXTRA_BINDING | UNKNOWN/WRONG_KIND |
|---|---:|---:|---:|---:|---:|
| default | 1552 | 71 | 386 | 45 | 0/0 |
| html | 1553 | 122 | 449 | 45 | 0/0 |

No recorte das 14 famílias autorizadas, os dois perfis são iguais: 14 owners em
MATCH, 169 membros presentes porém genericamente sem metadata bilateral, três
ausências obrigatórias (`color.spot`, `selector.before`, `selector.after`) e um
descendente desconhecido (`color.spot.tint`). A presença e o comportamento dos
169 paths autorizados foram decididos pelo oráculo focal, não promovendo
`UNVERIFIED_METADATA` por simples existência.

## Proveniência

A medição foi feita sobre o commit
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, com working tree não commitado.
`p1284-summary.json` congela o instante, `git status --short`,
`git diff HEAD --stat`, hashes de todos os inputs e os hashes dos dois produtos.
O vanilla ratificado é `a51e02804`, binário
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

## Gates locais

- `cargo test --workspace --offline --quiet`: 6.349 testes passaram, zero
  falhas; três ignorados.
- `cargo build --release -p typst-wiring --offline`: verde para o produto
  medido.
- `python3 -m unittest discover -s lab/surface-inventory -p 'test_*.py'`:
  22/22 verdes.
- `crystalline-lint . --quiet`: zero violações.
- `cargo fmt --all -- --check` e `git diff --check`: verdes.

## Ataques e veredito

O papel adversarial independente emitiu `ADV-P1284-v2`. As 35/35 mutações foram
`Rejected`, dando `mutation_score = 1.0`; 10/10 controles foram classificados
separadamente e ficaram fora do denominador. A evidência live incluiu 19/19
relações semânticas, 11/11 diagnósticos/casts negativos, 15/15 digests integrais
dos mapas, 169/169 paths autorizados `active_bilateral` nos dois perfis e
V15/V26 zerados. O recibo adversarial tem SHA-256
`3c8064546ab52c15f793583ab98246362f14098bf3bd3a7f4f398ce404d8141f`.

O verificador, que não escreveu contrato, candidata ou ataques, emitiu
`V-P1284-v1`: **Conformant**. O certificado confirma 23/23 pins L0 e ownership
1:1, 169/169 paths nos dois perfis, todos os oráculos, score mutacional 1.0,
controles e bloqueios sem crédito, residual P1285 separado e todos os gates
verdes. SHA-256 do certificado:
`7c6e47be02100eb4bf968f8d78b5cac7f1c32f285c4551c9823910628bd2903a`.

A segregação é procedimental e auditável; o filesystem compartilhado não
oferece atestação técnica de isolamento. O adversário vinculou cada mutação a
testemunhas discriminatórias reais, mas não produziu 35 builds mutantes
fisicamente isolados; essa limitação está explícita nos recibos e não foi
ocultada pelo score.

## Artefatos finais

| Artefato | SHA-256 |
|---|---|
| contrato C-P1284-v7 | `ea85130ae2954d0fb930e68ce82b5554780798ef5927bcab0cc7f109ab9c61f6` |
| recibo A-P1284-v5 | `1c04efc1c2100ecc47b7891cb7962beaca64895fc769c3ca509efc084196519d` |
| suíte A-P1284-v5 | `e8c6a0a2da6dbb784e29ba65e7cfa41b5eb17877e1f1f7fe5760e37d09f89d8f` |
| execução candidata | `d14ae6e871f5bb2efe421208774f00577d3142a7c09e1314c288535ee0fce431` |
| ataques ADV-P1284-v2 | `3c8064546ab52c15f793583ab98246362f14098bf3bd3a7f4f398ce404d8141f` |
| certificado V-P1284-v1 | `7c6e47be02100eb4bf968f8d78b5cac7f1c32f285c4551c9823910628bd2903a` |
| inventário default | `41a16a2856b335275656af64649f9bbd85465ff9ebf91c979e3398ae3650db4e` |
| inventário HTML | `2a3f5f7d19497f702e8a410c1a0a94a793e5a4420d4a98203ef2268bbabf37f3` |
| probes default | `da86beaf2f94dd21458f27f84696f47a621fd10c131ae3b1fb76ece4b0e211fb` |
| probes HTML | `828284bcb3f1676c52c9e32944506e4153656db7898b7b06ad07569463386db1` |
| resumo | `19f460ec8445b8869bf22f8354661d77c33e1ea0ffcb065a561dbc5f130a35f2` |
| residual P1285 | `4da83da19c0f97467286814faac79e6faacbfd46e3cf7436a226008ec73eb6fd` |
