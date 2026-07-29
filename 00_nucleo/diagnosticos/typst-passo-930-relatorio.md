# Relatório P930 — investigação do índice invertido por bloco Unicode

**Precede este passo:** `typst-passo-929-relatorio.md` — P929 concluiu que as
otimizações mecânicas (paralelizar o scan, cache em disco) não resolvem o
problema. Ficou a questão: existe uma forma de obter um índice invertido sem
pagar novamente o custo rejeitado em P928/P926?
**Objetivo deste passo:** responder a essa pergunta com medições: (A.1) qual o
custo de extrair `Coverage` durante a descoberta de fontes, aproveitando o
`Face` já aberto; (A.2) qual a cardinalidade real do índice invertido por bloco.

**Data:** 2026-07-29.
**Commit base:** `da18ea9f3` (P922–P924 integrados).
**Commit working tree:** `da18ea9f3` com alterações não commitadas de P927
(`03_infra/src/world.rs`, `04_wiring/src/main.rs`).

---

## Resumo executivo

A resposta ao gate é **não**: o índice invertido por bloco Unicode só é
obtido pagando um custo de arranque muito alto, e o ganho não é uniforme.

- **Custo de extrair coverage durante a descoberta:** `load_system_fonts()`
  passa de **~201 ms** para **~957 ms** — um acréscimo de **~756 ms** em cada
  arranque. Este é exactamente o custo que P880 decidiu evitar, e é superior
  ao tempo total de compilação dos cenários canônicos (~90 ms).
- **Cardinalidade do índice:** CJK reduz drasticamente (de 1112 para 32
  candidatos), mas emoji fica com 238 candidatos e grego com 866. A mediana
  geral é de 30 candidatos por bloco, com p95 de 505 e máximo de 1112.
- **Conclusão:** o índice invertido por bloco não é uma solução clara. O custo
  de construção é proibitivo para o caso comum, e a granularidade grosseira do
  bitmap por blocos mantém o conjunto de candidatos grande em scripts que
  partilham blocos com latim (grego, símbolos, emoji). Ficamos com o P927.

---

## Metodologia

- **Estado do código:** protótipos temporários em `03_infra/src/fonts.rs`
  (`font_info_from_bytes` preenche `coverage`), `01_core/src/entities/font_book.rs`
  (método `block_index_stats`) e `03_infra/src/fontdb.rs` (dois testes ignorados).
  Todos os protótipos foram revertidos ao final do passo.
- **Cenário:** descoberta de fontes do sistema via `fontdb` no ambiente de
  desenvolvimento (1112 faces).
- **Fase A.1:** `load_system_fonts()` foi cronometrado (a) sem extrair coverage
  (estado P927/P880) e (b) extraindo coverage a partir do `Face` já aberto.
- **Fase A.2:** construiu-se o índice invertido `bloco -> [font_ids]` a partir das
  `Coverage` e calcularam-se estatísticas de cardinalidade.
- **Ferramenta:** testes unitários `#[ignore]` executados com
  `cargo test -p typst-infra --lib ... -- --ignored --nocapture`.

---

## Fase A.1 — custo de extrair coverage durante a descoberta

`font_info_from_bytes` (`03_infra/src/fonts.rs:294`) já faz
`ttf_parser::Face::parse(data, index)` durante a descoberta de fontes
(`fontdb.rs:48-50`). A hipótese era que iterar a `cmap` do `Face` já em memória
fosse quase "de graça".

| Medição | `load_system_fonts()` | delta |
|---|---:|---:|
| Sem coverage (P927/P880) | **201 ms** | — |
| Com coverage (índice invertido possível) | **957 ms** | **+756 ms** |

**Conclusão:** iterar a `cmap` de 1112 fontes custa **~756 ms adicionais**, mesmo
com o `Face` já aberto. Isto torna a abordagem inviável para o caso comum: o
custo de arranque passa a ser mais de 8× o tempo total de compilação de um
documento simples (~90 ms).

---

## Fase A.2 — cardinalidade do índice invertido por bloco

Com a coverage preenchida, construiu-se o índice invertido:

```text
bloco 0x00   -> [font_id, ...]
bloco 0x01   -> [font_id, ...]
...
bloco 0xFFF  -> [font_id, ...]
```

### Estatísticas globais

| Métrica | Valor |
|---|---:|
| Faces no sistema | 1112 |
| Blocos populados | 599 |
| Mínimo de candidatos por bloco | 1 |
| Mediana | 30 |
| Média | 104,7 |
| p95 | 505 |
| Máximo | 1112 |
| Total de entradas (soma das listas) | 62 687 |

### Exemplos concretos

| Script | Codepoint | Bloco | Candidatos | Redução vs. scan total |
|---|---|---|---:|---:|
| Grego | U+03B1 (α) | 0x03 | 866 | 22% |
| CJK | U+4E2D (中) | 0x4E | 32 | 97% |
| Emoji | U+1F600 (😀) | 0x1F6 | 238 | 79% |

**Conclusão:**

- **CJK:** o índice é excelente — reduz de 1112 para 32 candidatos.
- **Emoji:** redução significativa (238), mas ainda há muitos candidatos.
- **Grego/símbolos:** a granularidade de 256 codepoints é demasiado grosseira.
  Muitas fontes latinas incluem alguns glyphs gregos ou símbolos, pelo que o
  índice mantém centenas de candidatos.

A mediana de 30 e o p95 de 505 mostram que, na maior parte dos blocos, o
índice ajuda pouco ou nada. O problema não é só o número total de fontes; é a
baixa especificidade do bitmap por blocos.

---

## Fase C — decisão

A ideia do índice invertido por bloco Unicode **não compensa** no estado actual:

1. **Custo de construção proibitivo:** ~756 ms de arranque, pior que a Opção 1
   rejeitada no P926 no caso comum.
2. **Ganho não uniforme:** ótimo para CJK, fraco para grego/símbolos, moderado
   para emoji.
3. **Causa raiz não atacada:** o bitmap por blocos de 256 codepoints é
   grosseiro. Sem uma estrutura mais fina (páginas, intervalos, Bloom filter),
   o índice mantém muitos falsos positivos.

**Decisão:** ficar com o P927. O custo absoluto de ~7 s para documentos com
CJK/emoji permanece como limitação aceite. Qualquer abordagem futura terá de
ou (a) usar uma estrutura de indexação mais fina que reduza os falsos
positivos sem iterar a `cmap` de todas as fontes, ou (b) aceitar o custo de
arranque de ~756 ms porque o workload típico do utilizador justifica (o que
não é o caso aqui).

---

## Fase D — estado da árvore

- `03_infra/src/fonts.rs`: revertido ao estado P927 (`coverage: Coverage::new()`
  em `font_info_from_bytes`).
- `01_core/src/entities/font_book.rs`: revertido — sem `block_index_stats` nem
  `BlockIndexStats`.
- `03_infra/src/fontdb.rs`: revertido — sem testes temporários.
- `cargo build --workspace --release`: ok.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente,
  `package_version_resolution.md`).

---

## Proveniência

- Commit base: `da18ea9f3`.
- Medição "sem coverage": teste temporário `p930_proto_baseline_load_system_fonts`
  (revertido).
- Medição "com coverage + estatísticas": teste temporário
  `p930_proto_custo_coverage_e_cardinalidade_indice` (revertido).
