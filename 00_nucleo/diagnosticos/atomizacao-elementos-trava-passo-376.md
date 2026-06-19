# Passo 376 — Atomização dos elementos (ADR-0110): relatório da Trava

> **Estado: PARADO NA TRAVA** (design-first, CLAUDE.md Regra de Ouro). ADR-0110 + claude.md + L0
> gravados; **nenhum código movido**. Aguarda aprovação do dono (desenho + escopo + hash).
> Caveat de stack: `RUST_MIN_STACK=33554432`. HEAD pós-P375 (`769ebf0a9`). Lint: 0 violações
> (1 warning V7 órfão no L0 novo — esperado pré-materialização, igual ao `f_fronteira_e1`).

---

## Estágio 0 — feito
- **ADR-0110** (`00_nucleo/adr/typst-adr-0110-atomizacao.md`): definição, não-metas, forma
  canónica, o erro histórico (P346 confundiu a métrica da lente com atomização). Já existia
  redigida (untracked); revista e mantida.
- **claude.md**: secção "Atomização (ADR-0110)" + entrada na tabela de ADRs Vigentes.

## Fase A — a medição (a fonte vence; `file:line`)

| Monólito | `file:line` | Tamanho | Exaustivo? |
|---|---|---|---|
| `layout_content` | `rules/layout/mod.rs:527-2383` | **1857 linhas, 59 arms bespoke** | sim, sem wildcard |
| walk introspect | `rules/introspect.rs:156` (+`:176`,`:422`,`:828`) | **43 arms** | sim |
| hub `content.rs` | `:1535-2317` | já delegação magra (P375) | n/a |

Arms de layout mais gordos (linhas): `Block` 296 · `Boxed` 198 · `Overline` 93 · `Stack` 84 ·
`Place` 66 · `Pad` 57 · `Transform` 49 · `Heading` 44 · `Columns` 44 · `Image` 41 · `Figure` 34 ·
`Shape` 33. [medido] Os arms **math** são agrupados e descem ao path `rules/math/layout/` — fora
desta fatia.

## O achado decisivo (§3 do L0) — o custo da forma canónica

Mover a lógica para o arquivo do elemento (a forma que a ADR-0110 desenha, **Opção A**) **exige
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

**Nenhum viola a ADR-0110** (sem `dyn`, sem wildcard; estático+exaustivo intactos). Mas são custo
real a aceitar conscientemente.

## O fork A/B (a decisão do dono)

- **Opção A — elemento-dono** (forma canónica ADR-0110): `impl HeadingElem { fn layout }` em
  `heading.rs`. Leitura por-elemento-único literal; **paga** o custo §3 (ciclo + `pub(crate)` +
  genéricos).
- **Opção B — layout por-elemento**: `rules/layout/elem/heading.rs`. Atomiza o monólito em ~59
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

## A Trava (PARA aqui)
**Para o dono aprovar:** (a) o desenho — Opção **A** ou **B**; (b) o escopo — a fatia-prova
`Heading` (ou outra); (c) o **hash** do L0 `rules/atomizacao_elementos.md` (guardar + calcular).
**Nenhum código movido antes.** Após aprovação → Estágio 1 (mover a fatia), em passo separado
(Trava 5: não emendar o seguinte).

Commit de fecho da Trava: ADR-0110 + claude.md + L0 + este relatório. Árvore (tracked) limpa fora
desses; backlog untracked intacto.
