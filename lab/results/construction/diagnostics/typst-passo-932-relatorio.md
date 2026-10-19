# Relatório P932 — Shaper: evitar carregamento de faces no fallback

**Precede este passo:** `typst-passo-927-relatorio.md` — a Opção 6 do P927
moveu o scan caro de coverage para um pré-carregamento condicional, mas o
custo absoluto do fallback CJK/emoji continuou na ordem dos ~7 s.
**Objetivo deste passo:** testar se o shaper consegue usar a cobertura já
calculada por `World::candidates_for_char` para evitar carregar/parsear faces
desnecessariamente, mantendo o caso comum rápido.

**Data:** 2026-07-29.
**Commit base:** `da18ea9f3` (inclui alterações não commitadas dos passos
P922–P930).
**Commit working tree:** `da18ea9f3` com alterações não commitadas em
`03_infra/src/shaper.rs` e `03_infra/src/fonts.rs` (protótipo P932).

---

## Resumo executivo

A resposta à pergunta de fundo — *"Temos como manter a performance do cristalino
no resto e melhorar o CJK/emoji?"* — é **sim**.

A variante **lazy** (cobertura calculada só quando o fallback dispara + shaper
a usar essa cobertura em vez de reabrir faces) consegue:

- **Caso comum:** zero regressão — rácios proto/original na banda de ruído
  (~1.01–1.05×).
- **CJK puro:** ~2.94× mais rápido (7.24 s → 2.46 s).
- **Emoji puro:** ~4% mais rápido (7.91 s → 7.59 s).
- **Documento misto `05-utf8`:** ~4% mais rápido (7.94 s → 7.60 s).

A variante **eager** (cobertura preenchida no arranque + mesma otimização do
shaper) provou que o ganho vem do shaper, mas mostrou também que o preço de
preencher a cobertura no arranque é inaceitável para o caso comum
(~1.23–1.59× mais lento). A combinação **lazy + shaper otimizado** é a que
resolve ambos os objectivos.

---

## Metodologia

- **Binário original:** `target-original/release/typst`, SHA-256
  `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072`
  (base P927).
- **Binário P932-proto:** `target/release/typst`, SHA-256
  `b8bd050cdc4013bde46ab554c2376874d677be7509470beefd462d180636c080`
  (cobertura eager no arranque + shaper otimizado).
- **Binário P932-lazy:** `target/release/typst`, SHA-256
  `b9f99a5caca0df150365d3d5f0fb05a9b7e25ab004ff87874b6349079244e760`
  (cobertura lazy P880/P927 + shaper otimizado).
- **Cenários canônicos:** os 7 documentos da frente (P872–P921), comparados
  `depois/antes`.
- **Cenários UTF-8:** `05-utf8.typ` e 4 inputs de bloco Unicode isolado
  (`utf8-latin`, `utf8-greek`, `utf8-cjk`, `utf8-emoji`).
- **Ferramenta:** `hyperfine`, warmup 5 / min-runs 20 para canônicos; warmup 2 /
  min-runs 10 para UTF-8.
- **Atestações:**
  - `tools/perf/results/p932-proto/attestation.json`
  - `tools/perf/results/p932-lazy/attestation.json`

---

## Fase A — impacto no caso comum (P932-lazy)

| Cenário | P927 (ms) | P932-lazy (ms) | rácio |
|---|---:|---:|---:|
| 01-hello | 88.29 | 92.93 | 1.05× |
| 02-lorem | 131.33 | 119.72 | 0.91× |
| 03-images | 100.45 | 99.94 | 0.99× |
| 04-math | 149.59 | 150.63 | 1.01× |
| 05-tables | 92.33 | 94.97 | 1.03× |
| 06-long | 293.95 | 295.46 | 1.01× |
| 07-context | 130.83 | 131.89 | 1.01× |

**Conclusão:** zero regressão prática. Todos os rácios ficam dentro da banda
de ruído habitual (~1.01–1.05×). O caso comum não paga o scan de coverage
porque o shaper otimizado continua a não disparar fallback para texto latino.

---

## Fase B — impacto nos casos UTF-8 (P932-lazy)

| Cenário | P927 (ms) | P932-lazy (ms) | rácio |
|---|---:|---:|---:|
| 05-utf8 | 7943.11 | 7603.10 | 0.96× |
| utf8-latin | 89.05 | 91.43 | 1.03× |
| utf8-greek | 89.10 | 91.66 | 1.03× |
| **utf8-cjk** | **7237.01** | **2459.93** | **0.34×** |
| utf8-emoji | 7905.91 | 7589.98 | 0.96× |

**Conclusão:**

- **Latim/grego puro:** sem alteração significativa (1.03×, dentro do ruído).
  Não há fallback, logo a otimização do shaper não entra em acção.
- **CJK puro:** melhoria dramática de ~2.94×. O shaper deixa de carregar uma
  face por candidato de fallback; a única parseagem que resta é a do scan
  lazy de coverage, pago uma vez no primeiro caractere CJK.
- **Emoji puro:** melhoria modesta de ~4%. O emoji dispersa-se por vários
  blocos e fontes, pelo que o bitmap de bloco ainda deixa passar muitos
  candidatos; mesmo assim, eliminar o segundo parse por fonte compensa.
- **`05-utf8` (misto):** ~4% mais rápido. O ganho no segmento CJK é parcialmente
  diluído pelas secções latinas e emoji do documento.

---

## Fase C — isolamento da causa (P932-proto, cobertura eager)

Para confirmar que o ganho vem do shaper e não do preenchimento eager de
coverage, testou-se a mesma otimização do shaper com a cobertura preenchida no
arranque (como no P931-proto).

| Cenário | rácio P932-proto / P927 |
|---|---:|
| 01-hello | 1.58× mais lento |
| 02-lorem | 1.44× mais lento |
| 03-images | 1.51× mais lento |
| 04-math | 1.31× mais lento |
| 05-tables | 1.51× mais lento |
| 06-long | 1.23× mais lento |
| 07-context | 1.34× mais lento |
| 05-utf8 | 1.00× |
| utf8-latin | 1.59× mais lento |
| utf8-greek | 1.59× mais lento |
| utf8-cjk | 0.35× (2.87× mais rápido) |
| utf8-emoji | 0.95× |

**Conclusão:**

- O preenchimento eager de coverage no arranque piora o caso comum em
  ~1.23–1.59×, porque todo o documento paga o scan da `cmap` das ~1100 fontes.
- O shaper otimizado melhora CJK mesmo com cobertura eager (2.87×), mas a
  regressão no caso comum é inaceitável.
- A solução viável é, portanto, **manter a cobertura lazy** e aplicar a
  otimização apenas no shaper.

---

## Fase D — implementação testada

### `03_infra/src/shaper.rs`

- `CandidateSet::fallback` passou de `Vec<Option<FontCandidate>>` para
  `Vec<Option<usize>>`. Isto evita que a simples filtragem de candidatos
  carregue e parseie faces.
- `load_fallback` deixou de chamar `FaceCache::get`/`ttf_parser::Face::parse`;
  agora só valida que o índice existe no `FontBook`.
- `covering_all` confia no filtro de `World::candidates_for_char` e não reabre
  a face para confirmar cobertura.
- `best_covering_run` para candidatos de fallback usa um cache local de
  `World::candidates_for_char` por caractere (`candidates_cache`), em vez de
  `face_covers_char`.
- A face só é carregada quando o candidato é seleccionado para shaping
  (`CandidateSet::get`).

### `03_infra/src/fonts.rs`

- Reverteu-se o preenchimento eager de `FontInfo.coverage` introduzido no
  P931-proto; a cobertura volta a ser `Coverage::new()` no arranque, conforme
  P880/P927. O scan caro continua a ser pago apenas quando o fallback
  realmente dispara.

---

## Fase E — validação

- `cargo test -p typst-infra --lib`: **743 passed, 0 failed**.
- `cargo check -p typst-wiring`: ok.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente,
  `package_version_resolution.md`).
- Documentos de teste (`utf8-cjk.typ`, `utf8-emoji.typ`, `01-hello.typ`)
  compilam para PDF sem erros.

---

## Conclusão

É possível **manter a performance do cristalino no caso comum e melhorar
significativamente o CJK/emoji**. A chave é:

1. Manter a cobertura Unicode **lazy** (P880/P927), para que documentos
   latinos não paguem nada.
2. Fazer o shaper **confiar nessa cobertura** durante a filtragem e extensão
   de runs de fallback, em vez de carregar uma face por candidato.

O protótipo lazy medido aqui é o caminho a seguir. O que falta para fechar o
passo é actualizar o `Prompt L0` de `shaper.md`, porque o código actual já não
confirma `face_covers_char` após `candidates_for_char` no caminho quente,
contrariamente ao especificado em `00_nucleo/prompts/infra/shaper.md:430-441`.

---

## Proveniência

- Commit base: `da18ea9f3`.
- Binário original (P927): `target-original/release/typst`, SHA-256
  `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072`.
- Binário P932-proto: `target/release/typst`, SHA-256
  `b8bd050cdc4013bde46ab554c2376874d677be7509470beefd462d180636c080`.
- Binário P932-lazy: `target/release/typst`, SHA-256
  `b9f99a5caca0df150365d3d5f0fb05a9b7e25ab004ff87874b6349079244e760`.
- Scripts: `tools/perf/benchmark-p932-proto.py` e
  `tools/perf/benchmark-p932-lazy.py`.
- Atestações:
  - `tools/perf/results/p932-proto/attestation.json`
  - `tools/perf/results/p932-lazy/attestation.json`
