# Relatório P927 — Opção 6: scan de coverage só quando o documento tem carácter não coberto

**Precede este passo:** `typst-passo-926-relatorio.md` — Opção 5 não resolveu o outlier
UTF-8; a Opção 6 (disparar o scan caro só quando o source bruto contém blocos
Unicode não cobertos pelas fontes embutidas) foi proposta como alternativa.
**Objetivo deste passo:** implementar a Opção 6, confirmar zero regressão no
caso comum e medição da melhoria nos casos UTF-8/CJK/emoji.

**Data:** 2026-07-28.
**Commit base:** `da18ea9f3` (P922–P924 integrados).
**Commit working tree:** `da18ea9f3` com alterações não commitadas em
`03_infra/src/world.rs` e `04_wiring/src/main.rs`.

---

## Resumo executivo

A Opção 6 **funciona conforme esperado** no objectivo principal:

- **Caso comum (7 cenários canônicos):** zero regressão — os rácios
  proto/original ficam na banda de ruído (~1.01–1.03×).
- **Texto dinâmico (`context`, interpolações, `read()`):** fica em scope-out
  explícito; esses casos continuam no caminho lazy original, sem regressão.
- **Latim/grego puro:** nenhum scan caro é disparado; o tempo mantém-se igual
  ao original (~87–90 ms).
- **CJK/emoji:** o scan dispara quando o documento precisa de fallback, mas o
  custo absoluto do scan lazy continua igual (~7 s). A Opção 6 elimina o
  custo para documentos sem fallback; não elimina o custo do fallback em si.

---

## Metodologia

- **Binário original:** `target-original/release/typst`, compilado a partir do
  worktree `/tmp/typst-crystalline-da18ea9f3` no commit `da18ea9f3`.
- **Binário protótipo:** `target/release/typst`, compilado a partir da working
  tree com a Opção 6.
- **Cenários canônicos:** os 7 documentos da frente (P872–P921), comparados
  `depois/antes`.
- **Cenários UTF-8:** `05-utf8.typ` e 4 inputs de bloco Unicode isolado
  (`utf8-latin`, `utf8-greek`, `utf8-cjk`, `utf8-emoji`).
- **Ferramenta:** `hyperfine`, warmup 5 / min-runs 20 para canônicos; warmup 2 /
  min-runs 10 para UTF-8.
- **Atestação:** `tools/perf/results/p927-canonical/attestation.json` e
  `tools/perf/results/p927-utf8/attestation.json`.

---

## Fase A — impacto no caso comum (7 cenários canônicos)

| Cenário | original (ms) | proto Opção 6 (ms) | rácio |
|---|---:|---:|---:|
| 01-hello | 88.26 | 90.28 | 1.02× |
| 02-lorem | 111.06 | 112.82 | 1.02× |
| 03-images | 94.29 | 97.39 | 1.03× |
| 04-math | 148.57 | 149.68 | 1.01× |
| 05-tables | 92.02 | 94.58 | 1.03× |
| 06-long | 292.72 | 296.14 | 1.01× |
| 07-context | 129.57 | 131.05 | 1.01× |

**Conclusão:** zero regressão prática. Todos os rácios ficam dentro da banda
 de ruído (~1.01–1.03×), muito abaixo do threshold de regressão observado na
Opção 1 (~1.12–1.48×).

---

## Fase B — impacto nos casos UTF-8

| Cenário | original (ms) | proto Opção 6 (ms) | rácio |
|---|---:|---:|---:|
| 05-utf8 | 7685.37 | 7739.57 | 1.01× |
| utf8-latin | 87.52 | 90.10 | 1.03× |
| utf8-greek | 87.29 | 90.62 | 1.04× |
| utf8-cjk | 7010.98 | 7074.91 | 1.01× |
| utf8-emoji | 7623.70 | 7479.94 | 0.98× |

**Conclusão:**

- **latim/grego:** nenhum scan caro é disparado; os tempos mantêm-se no
  mesmo nível do original (~87–90 ms).
- **CJK/emoji:** o scan ainda dispara, e o tempo total continua na ordem
  dos ~7 s. A Opção 6 **não reduz o custo absoluto do fallback** — apenas
  garante que esse custo só é pago quando o documento realmente precisa de
  fontes do sistema.
- O `layout_ms` medido interno baixou de ~6122 ms para ~5405 ms no
  `05-utf8`, porque parte do scan agora acontece fora do cronómetro de
  layout; o tempo de parede total, contudo, não melhorou (1.01×), o que
  confirma que o scan foi deslocado, não eliminado.

---

## Fase C — decisões de implementação

### Texto dinâmico fica em scope-out

Texto que só existe depois de `eval` (`context`, interpolação de variáveis,
conteúdo de `read()`/dados externos, `#for` sobre listas computadas) não é
visível no source bruto. Implementar um scan pós-`eval` implicaria alterar a
ordem do pipeline e expor `Content` a L3, pelo que foi decidido manter o
caminho lazy original para esses casos. Isto é uma **regressão zero**: o
comportamento é exactamente o mesmo de `da18ea9f3`.

### Scan do source bruto

O scan percorre a árvore de sintaxe (`SyntaxNode`) e considera apenas nós do tipo `Text`, `MathText` e `RawTrimmed`. Para cada caractere, verifica se o
bloco de 256 codepoints correspondente está na união das coberturas das
fontes embutidas (`<embedded>`). No primeiro caractere fora dessa união,
dispara `candidates_for_char`, que preenche o `coverage_cache` lazy.

### Testes unitários

- `p927_preload_coverage_latin_nao_dispara`: documento latino puro não
  popula o `coverage_cache`.
- `p927_preload_coverage_char_nao_coberto_dispara`: documento com caractere
  CJK faz com que o cache fique populado.

Ambos passam em `cargo test -p typst-infra --lib`.

---

## Conclusão

A Opção 6 cumpre o seu contrato: **evita o scan de coverage para documentos que
não precisam de fallback de fontes do sistema**, sem regressão no caso comum.
No entanto, é importante não confundir "evita" com "elimina": o outlier de
~7 s observado em `05-utf8` continua presente para documentos que realmente
contêm CJK/emoji, porque o scan caro ainda é necessário nesses casos. A
mudança deslocou o custo do caminho lazy para um pré-carregamento condicional;
não reduziu o custo absoluto do fallback. Qualquer passo futuro que queira
atacar o outlier de raiz terá de olhar para o algoritmo de fallback em si, não
para quando o scan é disparado.

## Fase D — estado da árvore

- `03_infra/src/world.rs`: adicionado `preload_coverage_if_needed`,
  `embedded_coverage_union` e `source_text_nodes`.
- `04_wiring/src/main.rs`: chamada a `preload_coverage_if_needed` depois de
  carregar o source principal e antes do dispatch por formato de saída.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente,
  `package_version_resolution.md`).
- `cargo test -p typst-infra --lib`: 745 passed, 0 failed.
- `cargo check -p typst-wiring`: ok.

---

## Proveniência

- Commit base: `da18ea9f3`.
- Binário original: `target-original/release/typst`, SHA-256
  `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072`.
- Binário protótipo: `target/release/typst`, SHA-256
  `f574a13fce0320ec72c6b53736dd17f16fd51e1c7fb61fe86c996502fae253d1`.
- Scripts: `tools/perf/benchmark-p927-canonical.py` e
  `tools/perf/benchmark-p927-utf8.py`.
- Atestações: `tools/perf/results/p927-canonical/attestation.json` e
  `tools/perf/results/p927-utf8/attestation.json`.
