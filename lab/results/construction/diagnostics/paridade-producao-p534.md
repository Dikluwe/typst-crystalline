# Paridade de Produção — P534

**Passo:** 534  
**Foco:** Fallback de fonte por carácter (multi-script).  
**Data:** 2026-07-02  

## Resumo

Antes de P534, o cristalino perdia caracteres não-latinos quando o documento
continha texto misto (latim + CJK + árabe) na mesma linha. O shaper (P515)
implementava fallback **apenas entre as famílias declaradas na `FontList`**; se
nenhuma delas cobrisse um caractere, esse caractere era renderizado como
`.notdef`/em branco. P534 corrige isto segmentando o texto por script Unicode e,
quando necessário, procurando fallback no `FontBook` global (fontes do sistema
carregadas via `fontdb`).

Emoji a cores (COLR/CPAL) permanece explicitamente fora do escopo — é um
problema de renderização de glifo, não de escolha de fonte.

## Sondas

### Sonda 1 — O que existia hoje

Ficheiros analisados:

- `03_infra/src/shaper.rs:206-247` — `split_run_by_font` iterava caractere a
caractere sobre `candidates` (famílias da `FontList`) e, se nenhuma cobrisse,
caía na primeira candidata (`covering.unwrap_or(0)`).
- `03_infra/src/shaper.rs:179-201` — `resolve_candidates` só resolvia famílias
declaradas; não consultava o `FontBook` global.
- `03_infra/src/pipeline.rs:403-530` — a resolução de fontes para embed
(`collect_fonts_from_doc` / `resolve_fonts`) também trabalha ao nível do
`(FontList, FontVariant)`, não por caractere.

Conclusão: P515 implementou *fallback intra-FontList*, não fallback global por
cobertura de glifo. A afirmação do handoff estava incorreta e foi corrigida no
relatório de P533 e neste.

### Sonda 2 — Referência vanilla

Ficheiro: `lab/typst-original/crates/typst-layout/src/inline/shaping.rs`
(linhas ~719-782, ~902-953).

O vanilla:

1. Segmenta o texto por **embedding level (BiDi)** e por **script Unicode** —
   `is_compatible` trata `Common`/`Inherited`/`Unknown` como genéricos.
2. Para cada segmento, itera as famílias declaradas e, se esgotadas e
   `fallback == true`, chama `book.select_fallback(..., text)` para obter uma
   fonte do sistema que cubra o segmento.
3. Faz shape do segmento com a fonte escolhida.

P534 adopta a mesma ideia geral (segmentação por script + escolha por
cobertura), mas com uma implementação mais simples: scan linear lazy sobre o
`FontBook` cristalino em vez de `select_fallback`/fontique.

### Sonda 3 — Impacto `BT...ET`

Ficheiro: `03_infra/src/export/stream.rs:188-288`.

Cada `FrameItem::TextShaped` emite o seu próprio bloco `BT...ET`; não existe
fusão de blocos consecutivos da mesma fonte. P534 aumenta o número de
`TextShaped` (um por segmento de script/fonte), logo aumenta o número de blocos
`BT...ET`. A fusão é **scope-out**; foi registada como item técnico futuro.

## Implementação

### Ficheiros alterados

- `00_nucleo/prompts/infra/shaper.md` — actualização do Prompt L0 com secção
  P534, hash corrigido via `crystalline-lint --fix-hashes`.
- `03_infra/Cargo.toml` — adicionada dependência `unicode-script`.
- `03_infra/src/shaper.rs`:
  - Novo `CandidateSet` com primárias (`FontList`) + fallback lazy sobre todo o
    `FontBook`.
  - Cache por caractere (`HashMap<char, usize>`) para evitar re-scans.
  - `split_run_by_font` segmenta por script Unicode (`is_compatible`) e por
    mudança de fonte.
  - Cada `FrameItem::TextShaped` transporta `style.font = família real usada`,
    para que o export multi-font embuta a face correcta.

### Testes novos

- `p534_split_run_by_font_respects_script_boundaries`
- `p534_shape_mixed_script_system_fallback`
- `p534_latin_only_stays_single_textshaped`

## Validação

### Documento de teste

```typst
#set text(font: "DejaVu Sans", size: 20pt)
Hello 你好 مرحبا
```

Comando:

```bash
./target/release/typst /tmp/test-mixed.typ /tmp/mixed-pos-fix.pdf
pdftotext -raw /tmp/mixed-pos-fix.pdf -
pdffonts /tmp/mixed-pos-fix.pdf
```

Resultado cristalino:

```text
Hello 你好‫مرحبا‬

name                    type        encoding  emb sub uni
--------------------    ----------  --------  --- --- ---
AAAAAA+CrystallineFont1 CID TrueType Identity-H yes yes yes
CrystallineFont2        CID TrueType Identity-H yes no  yes
```

- Latim, CJK e árabe são extraíveis.
- `pdffonts` lista duas fontes distintas (latim numa, CJK+árabe noutra —
  depende da ordem de descoberta do sistema).

### Comparação com Typst vanilla 0.15.0

```bash
/usr/local/bin/typst compile /tmp/test-mixed.typ /tmp/mixed-vanilla.pdf
pdftotext -raw /tmp/mixed-vanilla.pdf -
pdffonts /tmp/mixed-vanilla.pdf
```

Resultado vanilla:

```text
Hello 你好 ‫مرحبا‬

name                                 type        encoding  emb sub uni
------------------------------------ ----------  --------  --- --- ---
NPESBE+DejaVuSans                    CID TrueType Identity-H yes yes yes
FOKSOT+NotoSansCJKjp-Regular-...     CID Type 0C Identity-H yes yes yes
```

Observações:

- Vanilla mantém o texto numa única linha com espaços regulares.
- Cristalino renderiza o texto em duas linhas e com espaçamento diferente
  (posicionamento pós-layout não é objectivo deste passo).
- Visualmente, os três scripts são legíveis no PDF cristalino; o árabe é
  shaped RTL e ligado.

## Performance

Documento sem texto misto (`#lorem(200)`, DejaVu Sans):

```bash
hyperfine --warmup 1 --runs 5 \
  "./target/release/typst /tmp/bench-latin.typ /tmp/bench_crist.pdf" \
  "/usr/local/bin/typst compile /tmp/bench-latin.typ /tmp/bench_vanilla.pdf"
```

Resultado:

```text
Cristalino: 2.296 s ± 0.038 s
Vanilla:    2.518 s ± 0.050 s
```

Fases internas (média de 5 runs):

| eval | layout | shape | subset | render | total |
|-----:|-------:|------:|-------:|-------:|------:|
| ~5.7 ms | ~1.1 ms | ~435 ms | ~11 ms | ~12 ms | ~465 ms |

Para texto latino puro, o fallback global **não é activado**: cada caractere
resolve na primeira primária e fica em cache. Não há regressão mensurável em
relação ao custo anterior (parse da face primária por caractere já existia).

## Checklist de fecho

- [x] Sondas 1–3 completas antes do código.
- [x] Afirmação de P515 corrigida (fallback limitado à `FontList`).
- [x] Segmentação por script e escolha por cobertura implementadas.
- [x] Documento de teste latim+CJK+árabe funciona sem perda de caracteres.
- [x] Fontes de cor (emoji) registadas como scope-out separado.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p534.md`.

## Scope-out registado

- **Emoji a cores (COLR/CPAL):** requer renderização de glifo com múltiplas
  camadas; não é escolha de fonte. Item separado.
- **Fusão de blocos `BT...ET` consecutivos:** reduziria o overhead de
  múltiplos `FrameItem::TextShaped`, mas requer alteração ao `export/stream.rs`.
- **Ordenação sofisticada de fallback (fontique):** P534 usa ordem do
  `FontBook`; scripts complexos podem beneficiar de pesos por script no
  futuro.
