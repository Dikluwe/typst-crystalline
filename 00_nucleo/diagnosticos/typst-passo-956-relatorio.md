# Passo 956 — Relatório (modo verboso vanilla-espelhado como padrão de produção; modo compacto atrás de `--compact`)

**Data**: 2026-08-03
**Estado da árvore**: commit base `4f113026b` (P955); Fases 0+A commitadas em
`05b1e58fe`; Fases B–D por cima (working tree até ao commit final deste passo).

---

## 1. Fase 0 — ADR-0126 emendada (inversão verboso/compacto)

Correcção do dono aplicada **como emenda datada, texto original preservado**:

- §1: o formato Passo 20 (um `BT…ET` por item, `Td`) é o modo **compacto**;
  o modo **verboso** é o modo NOVO que espelha a semântica do vanilla (`Tm`,
  `q`/`cm`/`Q` por bloco, `cs`/`scn`, `Tr` explícito). Texto original (P954)
  preservado em bloco quotado marcado "rótulos INVERTIDOS".
- Destino final registado: verboso validado **directamente contra o vanilla**
  vira o **caminho de produção padrão**; compacto passa a **flag opcional**,
  validada por decalque contra o verboso, não removida.
- §2 (complemento P955): marcador de emenda inline; a conclusão de não-tensão
  mantém-se. §5: consequências reescritas na versão corrigida (texto original
  preservado). §4 sobrevive intacto à troca de rótulos.
- `adr/README.md`: linha da tabela e ledger actualizados.

## 2. Fase A — desenho (gate respeitado; dono confirmou com "continue")

Medição âncora (typst 0.15.1, `temp/p956/min-vanilla.pdf` + `rot-vanilla.pdf`,
`mutool clean -d`): por run de texto o vanilla emite

```pdf
q 1 0 0 -1 {x} {y_up} cm
/c0 cs … scn
BT 0 Tr /f0 11 Tf 1 0 0 -1 0 0 Tm [(…)] TJ ET
Q
```

com `/ColorSpace << /c0 [ICCBased gray] /c1 [ICCBased sRGB] >>` nos recursos e
`BDC`/`EMC` à volta (eixo acessibilidade — scope-out mantido). Em blocos
transformados (`#rotate`), a `cm` do bloco traz a transformação composta com o
flip (`R×F`) e o `Tm` é constante.

**Derivação chave** (registada em `stream.md` §P956): com `F = flip(1,0,0,-1)`
e `G` = cm de Group envolvente, o compacto desenha com matriz efectiva
`T(x,y)·G`; o verbose emite `cm = T(x,y)·F` e `Tm = F` → efectiva
`F·T(x,y)·F·G = T(x,y)·G` — **geometria idêntica por construção algébrica**,
top-level e local. O `y` emitido é exactamente o valor que já alimentava o
`Td` em cada caminho.

Desenho registado em 6 L0s (secção §P956 em cada): `stream.md` (envelope +
derivação + dispatch + regra de testes), `builder.md` (`/ColorSpace` na forma
array `[/ICCBased id]` — correcção do literal após nota do Agente B; ICC sRGB
sempre embutido em verbose; `PageContext` ganha `mode`), `mod.md`
(`pub enum StreamMode { Verbose, Compact }`, Default=Verbose; assinaturas com
`stream_mode` trailing — quebra deliberada, precedente P113), `pipeline.md`
(6 variantes `compile_to_pdf_bytes*`), `cli.md` (flag `--compact`; bool cru em
L2), `wiring.md` (tradução bool→StreamMode em L4).

## 3. Fase B — implementação (protocolo de dois agentes)

- **Agente A** (testes): 10 testes novos `p956_*` (envelope, ordem de
  operadores, equivalência de posição, isolamento `q…Q`, `cs`/`scn` por cor,
  `0 Tr`/`2 Tr` faux-bold, compact byte-igual ao Passo 20, filho de Group sem
  flip duplo, `/ColorSpace` presente/ausente por modo) + 248 call sites de
  teste actualizados com modo explícito (regra: asserções de bytes do formato
  P20 → `Compact`; o resto → `Verbose`). RED confirmado: 446 erros de
  compilação por API inexistente.
- **Agente B** (implementação): `StreamMode` + propagação (10 funções de
  export, 6 de pipeline), variantes verbose dos 3 emits com helpers
  partilhados (`push_tj_glyph_entries` — array TJ único nos dois modos;
  caminho compact refactorado **byte-inalterado**), `/ColorSpace` + ICC nos 3
  caminhos de página (alocação após fontes, convenção P777), flag `--compact`
  no CLI + wiring. Suíte verde: **5679 testes, 0 falhas** (4837 core + 760
  infra + 41 + 2 + 37 + 2). 3 testes antigos ajustados (liam PDF cru; o stream
  verbose cruza o limiar de compressão P884 — passaram a ler via
  `extract_page_content_streams_text`, asserções preservadas).
- **Ajuste pós-B (orquestrador)**: precisão do `cm` verbose subida de `{:.1}`
  para `{:.5}` (convenção P777 das matrizes de imagem; vanilla usa 5-8
  dígitos) — elimina arredondamento introduzido pelo lado verbose no decalque.
  Testes p956 actualizados; `cargo test -p typst-infra` 760 verdes.

## 4. Fase C — validação do verbose (padrão) contra o vanilla

Documento de 30 secções (`.typ/typst-math-comprehensive-test.typ`).

1. **Operador a operador** (`temp/p956/stream_diff.py`): a assinatura
   dominante dos blocos é idêntica à do vanilla —
   `q cm cs scn BT Tr Tf Tm TJ ET Q` (cristalino) vs a mesma sequência
   (vanilla, 1836/1917 blocos). Diferenças residuais com causa conhecida e
   fora do escopo: `2 Tr + w` de faux-bold (o vanilla usa fonte bold real —
   frente tipográfica pré-existente), agrupamento de vários runs num só `BT`
   no vanilla (o cristalino emite um bloco por run — granularidade, não
   semântica), `BDC`/`EMC` (scope-out), gray vs sRGB ICC (registado no L0).
2. **`compare.py` verbose vs vanilla**: mediana das medianas |dx| = **1.925pt**,
   med|dy| ≈ 0 em todas as secções. O modo verbose faz a ferramenta ler o
   nosso PDF como lê o vanilla — o emparelhamento melhora muito (ex.: sec 27:
   83 pares vs 15 com o stream compacto) e as medianas infladas por
   mis-pairing caem (sec 3: 13.3 → 0.066; sec 4: 27.8 → 0.741; sec 25: 27.6 →
   4.015). As secções ainda elevadas (8, 21, 22, 25, 28) são as divergências
   de conteúdo já catalogadas (P944 §8.3, P952 §6.4) — **nenhuma classe nova
   de divergência** (ver também o decalque exacto em §5.1: a geometria é a
   mesma do compacto, logo qualquer divergência residual vem do layout a
   montante, não do emissor).
3. **Benchmark** (hyperfine, 7 cenários, release; "antes" = binário de
   `05b1e58fe` em worktree; JSONs em `tools/perf/results/p956-canonical/`):

| Cenário | run 1 | run 2 |
|---|---|---|
| 01-hello | 1.102 | 1.021 |
| 02-lorem | 0.867 | 1.196 |
| 03-images | 0.986 | 0.980 |
| 04-math | 0.939 | 0.886 |
| 05-tables | 1.091 | 1.000 |
| 06-long | 1.017 | 0.953 |
| 07-context | 1.013 | 1.174 |
| **média** | **1.002** | **1.030** |

O spread não reproduz entre runs (médias absolutas do mesmo binário variam
±20% entre runs — ruído de ambiente, eval domina). Medição de fase via
`--timings-json` (06-long, 3 runs por binário): `render_ms` (a fase que este
passo toca) 22.3ms → 26.0ms (+17% da fase de export); `render_ms` é ~6% do
total → **custo do verbose ≈ +1% do tempo total**, dentro do ruído do
benchmark de ponta-a-ponta. `eval`/`layout`/`shape` inalterados.

## 5. Fase D — modo compacto (flag) validado por decalque

1. **Decalque exacto por bloco** (parser próprio sobre os streams
   descomprimidos, `temp/p956/`): **2076 blocos de texto nos dois PDFs, mesma
   ordem, max |Δposição| = 0.05pt** — e esse resíduo vem do lado COMPACTO
   (os caminhos legados `Td` `{:.1}` de `emit_text_pdf`/`emit_glyph_pdf`,
   pré-P956); o verbose é estritamente mais preciso (`{:.5}`). A equivalência
   geométrica F·F=I fica assim confirmada empiricamente.
2. **Tamanho** (a razão de existir da flag — medido nos 7 cenários + doc de
   30 secções, debug binary; tamanho é propriedade do modo, não do build):

| Cenário | verbose | compact | compact poupa |
|---|---|---|---|
| 01-hello | 4927 | 4406 | 10.6% |
| 02-lorem | 17551 | 16534 | 5.8% |
| 03-images | 5450 | 4847 | 11.1% |
| 04-math | 11591 | 10437 | 10.0% |
| 05-tables | 7326 | 6671 | 8.9% |
| 06-long | 460060 | 418267 | 9.1% |
| 07-context | 10921 | 9917 | 9.2% |
| 30 secções | 158203 | 153687 | 2.9% (vanilla: 150694) |

A flag `--compact` oferece PDFs **~3-11% menores** em troca de menos
semântica de operadores (sem `Tm`/`cm` por bloco, sem `cs`/`scn`, sem `Tr`
explícito). Documentado no `--help` (docstring clap), em `cli.md` §P956 e
aqui — a troca exacta fica visível para quem decidir usar a flag.
3. Nota de raster: diff pixel a 150dpi entre os dois modos = 1.17% de pixeis
   com diferença (bordas de glifos; 0.012% com flip total) — comportamento
   sub-pixel do rasterizador perante CTM com flip, idêntico ao que o vanilla
   apresenta; crops a 8× são visualmente idênticos. `verbose vs vanilla` tem
   MENOS diferença de raster que `compact vs vanilla` (4201 vs 4265 px na
   linha de teste a 300dpi).

## 6. Validação final

- `cargo test --workspace`: **5679 testes, 0 falhas** (Após B); re-run
  `cargo test -p typst-infra` após o ajuste de precisão: 760 verdes.
- `crystalline-lint .`: **zero violations** (hashes resselados; resta só o V7
  órfão pré-existente alheio).
- Smoke: `typst doc.typ` (sem flag) → verbose; `typst doc.typ --compact` →
  formato Passo 20; `--help` documenta a flag.

## 7. Registos e pendências

- `emit_bitmap_glyph_draws` (CBDT/emoji) inalterado nos dois modos (desenho de
  imagem, não texto) — decisão registada em `stream.rs`.
- Refinamento futuro possível (não bloqueante): colour space ICC **gray** para
  preenchimentos acromáticos, como o vanilla (`builder.md` §P956 nuance).
- O vanilla agrupa vários runs de texto da mesma linha num único `BT…ET`;
  adoptar esse agrupamento reduziria ainda mais o delta de tamanho face ao
  vanilla — candidato a passo próprio se o tamanho do padrão verbose pesar.
- Pendências pré-existentes fora do escopo (conteúdo `lr` literal, gregos
  literais, faux-bold vs fonte bold real) permanecem como estavam — nenhuma
  agravada nem corrigida por este passo.
