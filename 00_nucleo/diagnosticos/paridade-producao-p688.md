# Paridade Produção — P688 — Re-teste de `cetz` com versão e padrão correctos

**Estado:** fechado (sonda directa; **sem alteração de código**). Conclusão anterior
("vanilla também falha em cetz", P686/P687) **corrigida**: era artefacto do documento
de teste (versão antiga 0.2.2 + padrão `draw.line` shorthand + inteiros nus). Com a
versão actual (0.5.2) e o padrão documentado, o **vanilla renderiza cetz**. O cristalino
falha num bloqueio **real e localizado** (`str.codepoints()` em falta, via `oxifmt`).

**Commit do trabalho:** `__P688_COMMIT__`
**Commit base (HEAD antes deste passo):** `45b4af30900be55032ddb4dd21b403c6a645205f`
**Hora da medição:** `2026-07-10T17:06:38-03:00` (saída de `date -Is`)
**Árvore:** detached HEAD; **zero** ficheiros tracked alterados (sonda só produz relatório).

---

## Proveniência da medição (regra de P569/P574)

- **Código medido:** este commit `__P688_COMMIT__` (= base `45b4af309`; sem diff de código).
- `git diff HEAD --stat`: vazio (nenhum tracked alterado).
- Binários: cristalino `target/debug/typst` (CLI posicional); vanilla
  `lab/typst-original/target/release/typst` = `typst 0.15.0 (969087ec)` (`compile`).
- **Rede usada (reversível, só cache local):** descarreguei
  `https://packages.typst.org/preview/cetz-0.5.2.tar.gz` (HTTP 200) para
  `~/.cache/typst/packages/preview/cetz/0.5.2/`; o vanilla, ao compilar, descarregou a
  dependência `@preview/oxifmt:1.0.0` para a mesma cache. Nada foi alterado no repo.

---

## Sonda (medida antes de decidir — ADR-0108)

### 1. Manifestos — compatibilidade declarada

| pacote | `compiler` (mínimo) | observação |
|--------|---------------------|------------|
| cetz 0.2.2 | `0.10.0` | antigo; sem plugin WASM |
| cetz 0.5.2 | `0.14.0` | actual; **inclui `cetz-core/cetz_core.wasm`** (plugin) |

Ambos ≤ vanilla 0.15.0 → a **versão** não era a incompatibilidade. O que falhou em
P686/P687 (doc com `0.2.2` + `#import "..: canvas, draw"` + `draw.line((0,0),(1,1))`)
era o **padrão do documento**, não cetz em si.

### 2. Vanilla com versão/padrão correctos — RENDERIZA

Documento (padrão oficial da documentação de cetz):

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

- `vanilla compile` → **exit 0**, PDF 2769 bytes; `mutool` → PNG 14759 bytes.
- Imagem observada (`temp_p688/vanilla.png`, 1241×1754): uma **linha** diagonal de
  ~(205,207) a ~(322,147) e um **círculo** centrado em ~(205,207) raio ~58 px — exactamente
  `line((0,0),(2,1))` + `circle((0,0))`, no topo da página A4.
- **É a primeira confirmação nesta cadeia de que cetz produz PDF visual no vanilla.**

### 3. Cristalino no mesmo documento corrigido — bloqueio REAL

```
error: field access não suportado em str   (em <detached>, exit 1, sem PDF)
```

Localização (file:line, não inferida): o pacote `oxifmt 1.0.0` (dependência de cetz)
executa **no import** a linha
`~/.cache/typst/packages/preview/oxifmt/1.0.0/oxifmt.typ:12`:
`#let using-090 = using-080 and str(-1).codepoints().first() == _minus-sign`.
O cristalino não implementa `str.codepoints()` → o dispatcher de métodos de `str`
(`01_core/src/rules/stdlib/collections.rs:72-89`) devolve `None` e `eval/closures.rs`
reporta `field access não suportado em str`. `import "@preview/oxifmt:1.0.0"` **sozinho**
já reproduz o mesmo erro — logo o bloqueio é na **dependência**, não no desenho.

**Reproduzido minimalmente (vanilla=0 / cristalino=1):**

| snippet | vanilla | cristalino |
|---------|---------|------------|
| `#("abc").len()` | 0 | **0** (suportado) |
| `#("abc").at(0)` / `.split("-")` / `.clusters()` | 0 | **0** (suportado) |
| `#("abc").codepoints()` | 0 | **1** — `field access não suportado em str` |
| `#("abc").position("b")` | 0 | **1** — idem |
| `#("abc").match(regex("b"))` | 0 | **1** — idem |
| `#red.components()` | 0 | **1** — `field access não suportado em color` (P687) |
| `#("abc").to-bytes()` | 1 | 1 (não é gap — vanilla também não tem) |

Métodos de `str` **implementados** no cristalino: `len, first, last, at, slice, clusters,
contains, starts-with, ends-with, find, replace, trim, split, repeat, to-upper, to-lower,
to-unicode, rev`. **Em falta** (usados por oxifmt/cetz): `codepoints` (13× em oxifmt),
`position`, `match`, e (família) field/method access em `color` (P687).

**Classificação (ADR-0107):** o conjunto de métodos de `str` é **semântica da
linguagem** → paridade exigida; a *forma* do dispatcher (match em `collections.rs`) é
**mecânica**. Aceitação ao nível da língua (o método existe e devolve o valor certo).

---

## Decisão (conforme a árvore do passo)

O vanilla **funciona** com a versão/padrão correctos; o cristalino falha num ponto onde
o vanilla (agora com sucesso) **não** falha → esse é o **próximo bloqueio real**, não um
artefacto do documento: **métodos de `str` em falta (`codepoints`/`position`/`match`),
expostos via `oxifmt` no import de cetz**. É um gap de stdlib independente de cetz —
resolve-se num passo próprio (implementar `str.codepoints` e afins), fora do âmbito
desta sonda.

A conclusão anterior "vanilla também falha em cetz" (P686/P687) fica **revogada**: era
artefacto do documento de teste antigo. Não será repetida sem a reconfirmação aqui feita.

## Débitos (fora de escopo)

- `str.codepoints()` / `str.position()` / `str.match()` (e field/method access em
  `color`, P687) — métodos da linguagem por implementar; bloqueiam cetz via oxifmt.
- cetz 0.5.2 usa plugin WASM (`cetz_core.wasm`); o cristalino muito provavelmente não
  suporta `plugin()` — bloqueio **posterior** ao de `str.codepoints`, a confirmar só
  depois de ultrapassar o import (não medido ainda; marcado como inferência — refutado
  se, após implementar os métodos de `str`, o erro seguinte não for de plugin).
- A imagem de validação (`temp_p688/vanilla.png`) foi observada e descrita; o scratch
  `temp_p688/` é removido no fim (não se commita binário).

## Conclusão

cetz **é** renderizável pelo vanilla 0.15.0 com a versão 0.5.2 e o padrão documentado;
a validação de cetz passa a ter terreno comparável. O bloqueio do cristalino é real,
localizado (`oxifmt.typ:12` → `str.codepoints()` em falta) e independente de cetz —
próximo passo óbvio: implementar os métodos de `str` em falta. Medido com proveniência
(ADR-0108); paridade avaliada ao nível da língua (ADR-0107).
