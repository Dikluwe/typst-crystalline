# Passo 376 — Atomização dos elementos (ADR-0109): relatório

> **Estado: MATERIALIZADO — fatia família-containers.** Trava aprovada pelo dono (Opção B + fatia
> containers); Estágio 1 executado. Caveat de stack: `RUST_MIN_STACK=33554432`. HEAD pós-P375
> (`769ebf0a9`). **Suíte verde** (2747+472+24+21+2, 0 falhas; rede de caracterização `+11` sem
> asserção virada); **lint 0/0**; build limpo.

## Estágio 1 — a fatia materializada (Opção B)

A lógica de layout dos 4 containers movida do monólito para o seu arquivo (free function
`pub(super) fn layout<M,S>(layouter, e)`, mesma módulo-árvore `rules::layout` — acessa o estado
privado do `Layouter` por ser módulo descendente; **sem** import reverso, **sem** `pub(crate)`):

| Elemento | Arm antes (mod.rs) | Arquivo novo | Linhas |
|---|---|---|---|
| `Block` | `Content::Block(e) => block::layout(self, e)` | `engine/layout/block.rs` | 302 |
| `Boxed` | `Content::Boxed(e) => boxed::layout(self, e)` | `engine/layout/boxed.rs` | 202 |
| `Stack` | `Content::Stack(e) => stack::layout(self, e)` | `engine/layout/stack.rs` | 87 |
| `Pad`   | `Content::Pad(e) => pad::layout(self, e)`     | `engine/layout/pad.rs`   | 75 |

**Métrica de leitura (ADR-0109):** `layout_content` encolheu **1857 → 1276 linhas** (−581);
`layout/mod.rs` **2867 → 2293** (−574). Cada container é agora legível no seu arquivo.

**Não-metas confirmadas (medido):** `match` exaustivo MANTIDO (0 wildcards); despacho ESTÁTICO
(0 `dyn`/vtable de elemento — o único `Box<dyn Iterator>` em `stack.rs` é o iterador original do
Stack, não despacho); `entities/` **não tocado** → `content→elements` inalterado, sem ciclo.
**Linhagem:** os 4 arquivos com `@prompt rules/atomizacao_elementos.md` + `@prompt-hash` (V7
órfão resolvido). **Perf:** free function inlinável (sem regressão esperada; suíte 0.37s estável).

---

## Estágio 0 — feito
- **ADR-0109** (`00_nucleo/adr/typst-adr-0109-atomizacao.md`): definição, não-metas, forma
  canónica, o erro histórico (P346 confundiu a métrica da lente com atomização). Já existia
  redigida (untracked); revista e mantida.
- **claude.md**: secção "Atomização (ADR-0109)" + entrada na tabela de ADRs Vigentes.

## Fase A — a medição (a fonte vence; `file:line`)

| Monólito | `file:line` | Tamanho | Exaustivo? |
|---|---|---|---|
| `layout_content` | `engine/layout/mod.rs:527-2383` | **1857 linhas, 59 arms bespoke** | sim, sem wildcard |
| walk introspect | `rules/introspect.rs:156` (+`:176`,`:422`,`:828`) | **43 arms** | sim |
| hub `content.rs` | `:1535-2317` | já delegação magra (P375) | n/a |

Arms de layout mais gordos (linhas): `Block` 296 · `Boxed` 198 · `Overline` 93 · `Stack` 84 ·
`Place` 66 · `Pad` 57 · `Transform` 49 · `Heading` 44 · `Columns` 44 · `Image` 41 · `Figure` 34 ·
`Shape` 33. [medido] Os arms **math** são agrupados e descem ao path `rules/math/layout/` — fora
desta fatia.

## O achado decisivo (§3 do L0) — o custo da forma canónica

Mover a lógica para o arquivo do elemento (a forma que a ADR-0109 desenha, **Opção A**) **exige
três coisas que a ADR não precificou** — medido no arm `Heading` (`:758-798`), que lê o estado
**privado** do `Layouter`:

1. **Import reverso `entities → rules::layout::Layouter`** → novo **ciclo de módulos**
   `entities ↔ rules` dentro de L1. Hoje **nenhum** elemento chama o `Layouter` (as menções em
   `math_*.rs` são comentário) [medido]. Mecanicamente OK (Rust aceita ciclo intra-crate; lint
   fica 0/0), mas inversão estrutural nova.
2. **Alargar visibilidade do `Layouter`** (`style`/`regions`/`font_size_pt`/`flush_line`/
   `layout_content` privados → `pub(crate)`).
3. **Threading de genéricos** `Layouter<'a, M: FontMetrics, S: ImageSizer>` → o método do elemento
   fica genérico em `M,S`.

**Nenhum viola a ADR-0109** (sem `dyn`, sem wildcard; estático+exaustivo intactos). Mas são custo
real a aceitar conscientemente.

## O fork A/B (a decisão do dono)

- **Opção A — elemento-dono** (forma canónica ADR-0109): `impl HeadingElem { fn layout }` em
  `heading.rs`. Leitura por-elemento-único literal; **paga** o custo §3 (ciclo + `pub(crate)` +
  genéricos).
- **Opção B — layout por-elemento**: `engine/layout/elem/heading.rs`. Atomiza o monólito em ~59
  arquivos pequenos **sem** o custo §3 (segue a separação domínio/render do Typst vanilla); a
  lógica fica ao lado do layout, não do struct.

Ambas mantêm `match` exaustivo + estático + imports → ambas satisfazem as não-metas.

**Recomendação (a refutar):** **fatia-prova de 1 elemento (`Heading`) em Opção A**, para medir o
custo §3 concreto numa unidade antes do rollout de 59; se o ciclo for indesejável, **Opção B**
atinge a leitura sem o custo. [inferência marcada]

## Não-metas confirmadas
`match` exaustivo MANTIDO · despacho ESTÁTICO (ADR-0026 satisfeita) · imports FICAM
(`content→elements=68` não-gate) · content-preserving (rede +11 oráculo) · α/caso 2/caso 4/flag/
F-5b INTACTOS.

## Decisão do dono (P376) — resolvida
- **Forma: Opção B** — layout por-elemento em `engine/layout/<elem>.rs` (flat, seguindo os 6
  precedentes existentes: `figure.rs`/`image.rs`/`grid.rs`/…), sem ciclo, sem `pub(crate)`, sem o
  custo §3. (Opção A fica registada como alternativa.)
- **Escopo: família containers** — `Block`+`Boxed`+`Stack`+`Pad`. Materializado no Estágio 1 (topo
  deste relatório).

## Commits
| Estágio | Commit |
|---|---|
| 0 — ADR-0109 + claude.md | `4461c808a` |
| Trava — Opção B + containers aprovados | `9bed75348` |
| 1 — materialização da fatia | `cf204f949` |

## Notas de execução (transparência)
- O ficheiro da **ADR-0109 desapareceu** da working tree a meio do passo; restaurado de `HEAD`
  (estava commitado, intacto).
- O texto do L0 **não fixa o valor do hash** (auto-invalida-se a cada edição); referência genérica
  + `crystalline-lint --fix-hashes`.
- Desvio menor face ao §4/§5 do L0: arquivos **flat** em `layout/` em vez de subdir `elem/` —
  alinha com os 6 precedentes do repo; L0 corrigido para refletir.

**Fora de escopo (Trava 5):** os lotes futuros (Figure/Image/Shape/Transform e o resto), a
varredura do projeto inteiro e a decisão de crates — decisão do dono, passos separados.
