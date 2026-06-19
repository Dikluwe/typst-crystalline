# Prompt L0 — Atomização dos elementos (layout/introspect → arquivo do elemento)

Hash do Código: pendente — definido na materialização (Estágio 1, pós-Trava, hash humano por fatia)

**Camada**: L1 · **Módulos afetados**: `01_core/src/rules/layout/mod.rs` (o monólito
`layout_content`), `01_core/src/rules/introspect.rs` (o walk), e os arquivos dos elementos
`01_core/src/entities/elements/<elem>.rs`.
**Decisão de origem**: **ADR-0110** (atomização — mover a lógica para o arquivo da unidade;
`match` exaustivo + estático + imports ficam; NÃO é desacoplar `content→elements`). Complementa
ADR-0026 (enum fechado sem vtable, **satisfeita**) e ADR-0105 (modelo D, exaustividade cl.3,
**mantida**). Disciplina: ADR-0107 (content-preserving) + ADR-0108 (medir antes de decidir).
**Tipo**: especificação de atomização. **Forward-looking**: descreve o desenho + a medição; a
implementação é **por fatias, pós-Trava**, com **hash humano por fatia**. **Content-preserving**:
a lógica muda de arquivo, **não** muda de comportamento (oráculo: a rede de caracterização +11,
P331).

> **Estatuto: EM TRAVA (P376) — aguarda aprovação do dono.** Nenhum código movido antes da
> aprovação do desenho + do escopo + do hash (CLAUDE.md, Regra de Ouro). Este L0 regista a
> medição e o desenho; a escolha A/B (§4) e a fatia (§5) são do dono.

---

## §0 — Estatuto e sincronização de hash

Em Trava. O **primeiro lote** (a fatia-prova, §5) materializa-o: o código declara
`@prompt rules/atomizacao_elementos.md` + `@prompt-hash <hash deste ficheiro>`, e
`crystalline-lint --fix-hashes .` sincroniza. Até lá o ficheiro é órfão (V7) — esperado e
idêntico ao padrão do `f_fronteira_e1.md` pré-P334.

---

## §1 — A medição do monólito (a fonte vence; `file:line`)

### `layout/mod.rs` — `fn layout_content` (`01_core/src/rules/layout/mod.rs:527-2383`)
- **1857 linhas, 59 arms bespoke, SEM wildcard (exaustivo).** [medido P375 + P376]
- Os arms mais gordos (linhas): `Block` 296 (`:1798`), `Boxed` 198 (`:1600`), `Overline` 93
  (`:2220`), `Stack` 84 (`:1516`), `Place` 66 (`:1266`), `Pad` 57 (`:1418`), `Transform` 49
  (`:1006`), `Heading` 44 (`:758`), `Columns` 44 (`:2118`), `Image` 41 (`:1215`), `Figure` 34
  (`:935`), `Shape` 33 (`:973`). [medido]
- Os arms math (`MathAccent`/`MathCancel`/… `:891-920`) são **agrupados** e descem ao path math
  (`rules/math/layout/`), fora deste monólito — **fora desta fatia.** [medido `:903`]

### `introspect.rs` — o walk (`01_core/src/rules/introspect.rs:156`)
- **43 arms nativos**; cada elemento aparece também em `materialize_time` (`:176`),
  `extract_payload` (`:422`), e o walk principal (`:828`). [medido]

### O hub (`content.rs`)
- Já é **delegação magra** (os 6 métodos `is_empty/plain_text/eq/get_field/map_content/map_text`
  delegam a `e.metodo()` via a infra que o P375 mediu já existir). **Nada gordo a atomizar no hub
  além da delegação que já está lá.** [medido P375]

### O que os arms leem (a lógica é inteira, não um pedaço) — ex.: `Heading` (`:758-798`)
Lê o **estado privado do `Layouter`**: `self.font_size_pt`, `self.style`, `self.regions`,
`self.page_config.margin`, `self.chain` (`custom("heading.numbering")`), `self.current_location`,
`self.introspector` (`formatted_counter_at`), e chama `self.flush_line()` + `self.layout_content()`.
[medido `:769-797`] **A lógica depende inteiramente do estado local do layouter** — mover exige
passar/expor esse estado (ver §3).

---

## §2 — A forma canónica da delegação (ADR-0110)

Antes (monólito): `Content::Heading(h) => { /* 44 linhas */ }`.
Depois (magro, exaustivo, estático): `Content::Heading(h) => h.layout(lo)`, com a lógica num
método do elemento. **O `match` fica magro (1 linha/arm); a jump table, a exaustividade e os
imports ficam.**

---

## §3 — O custo medido da forma canónica (Opção A) — o achado para o dono

Mover a lógica para `entities/elements/heading.rs` (a forma que a ADR-0110 desenha) **exige três
coisas que a ADR não precificou** (medido, marcado para o dono decidir — ADR-0108):

1. **Import reverso `entities → rules::layout::Layouter`.** Hoje **nenhum** elemento chama o
   `Layouter` (as menções em `math_*.rs` são comentário) [medido]. Cria um **ciclo de módulos**
   `entities ↔ rules` **dentro de L1** — mecanicamente permitido (Rust aceita ciclos intra-crate;
   `crystalline-lint` opera na topologia de **camadas**, não em ciclos intra-L1 → fica **0/0**),
   mas é uma inversão estrutural nova.
2. **Alargar a visibilidade do `Layouter`.** Os campos lidos (`style`, `regions`, `font_size_pt`,
   …) e métodos (`flush_line`, `layout_content`) são privados [medido `:84-218`]. O método no
   elemento exige `pub(crate)` neles — **alarga a superfície de API** do layouter.
3. **Threading de genéricos.** `Layouter<'a, M: FontMetrics, S: ImageSizer>` é genérico
   [medido `:84`] → o método vira `fn layout<M: FontMetrics, S: ImageSizer>(&self, lo: &mut
   Layouter<'_, M, S>)` em cada elemento.

**Nenhum desses é violação da ADR-0110** (não exige `dyn` nem wildcard; o despacho fica estático e
exaustivo). Mas os três são **custo real** que o dono deve aceitar conscientemente.

---

## §4 — O fork A/B (decisão do dono na Trava)

- **Opção A — elemento-dono (forma canónica da ADR-0110).** `impl HeadingElem { fn layout(&self,
  lo) }` em `heading.rs`. **Prós:** "abrindo só o arquivo do elemento" no sentido literal — struct
  + Element + layout + introspect juntos. **Contras:** o custo §3 (ciclo + `pub(crate)` +
  genéricos).
- **Opção B — arquivo de layout por-elemento.** `rules/layout/elem/heading.rs` com
  `pub(super) fn layout<M,S>(lo, h: &HeadingElem)`. **Prós:** atomiza o monólito de 1857 linhas em
  ~59 arquivos pequenos **sem** o ciclo nem o alargamento de visibilidade (mesmo módulo-árvore;
  segue a separação domínio/render do Typst vanilla). **Contras:** a lógica de layout do elemento
  vive ao lado do layout, **não** no mesmo arquivo da definição do struct (a leitura é por-feature,
  não por-elemento-único).

**Ambas mantêm o `match` exaustivo + estático + os imports** → ambas satisfazem as não-metas da
ADR-0110. A diferença é **onde** o arquivo atomizado mora e o custo §3.

**Recomendação (a refutar pelo dono):** começar pela **Opção A numa fatia-prova de 1 elemento**
(§5) para **medir o custo §3 concretamente** numa unidade antes do rollout de 59; se o ciclo for
indesejável, **Opção B** atinge a métrica de leitura da ADR-0110 sem o custo §3. [inferência —
marcada; refuta-se medindo a fatia-prova real]

---

## §5 — Escopo e ordem (a fatia-prova; válvula da ADR-0110)

- **Fatia-prova: `Heading`** — 44 linhas de layout (`:758`) + introspect (`materialize_time :176`,
  walk `:828`), arquivo do elemento já existe e é pequeno (`heading.rs`, 124 linhas). Prova a forma
  end-to-end e mede o custo §3 numa unidade. **Aditivo-neutro**: a rede +11 passa sem virar.
- **Depois** (pós-fatia-prova aprovada): as famílias por lógica decrescente — containers
  (`Block`/`Boxed`/`Stack`/`Pad`), depois `Figure`/`Image`/`Shape`/`Transform`, depois o resto.
- **Fora desta fatia**: os arms math (path `rules/math/layout/`, agrupados); a varredura do projeto
  inteiro; a decisão de crates (todas decisão do dono, **depois**).

---

## §6 — Não-metas confirmadas (ADR-0110)

- **`match` exaustivo MANTIDO** (sem wildcard) — a garantia do compilador fica. [medido: os 59
  arms são sem wildcard hoje]
- **Despacho ESTÁTICO** — sem `dyn`/vtable/PropMap. A ADR-0026 fica **satisfeita**, não emendada.
- **Imports FICAM** — `content→elements = 68` (P374) **não é gate** e não se mexe; a Opção A
  **adiciona** um import reverso (§3.1), a Opção B não.
- **Content-preserving** — a rede de caracterização +11 e a suíte passam **sem asserção virada**;
  qualquer viragem = a lógica mudou ao mover → **investigar, não mascarar**.
- **INTACTOS**: α/caso 2, `morph_canon`/`==`, caso 4, flag P350c, F-5b (fechado), os 3 numbering,
  o `#set` de props de usuário.
