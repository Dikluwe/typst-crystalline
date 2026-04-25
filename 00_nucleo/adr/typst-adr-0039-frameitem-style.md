# ⚖️ ADR-0039: Forma de estilo no `FrameItem::Text`

**Status**: `EM VIGOR`
**Validado**: Passo 100.E — 783 testes; +3 integração `Content::Styled` end-to-end; zero violations.
**Data**: 2026-04-23
**Autor**: Humano + IA
**Passo associado**: 100

---

## Contexto

ADR-0038 (Passo 99) materializou `Style`/`Styles`/`StyleChain` e
`Content::Styled` em L1, com decisão COEX (coexistência com
`TextStyle` plano). DEBT-48 ficou aberto para fechar o circuito:
substituir a representação plana de estilo no output do Layouter
(`FrameItem::Text.style`) por uma forma consistente com `StyleChain`.

O inventário 100.A contou:

- **55 sítios** de leitura de `.size` em `FrameItem::Text.style` /
  `Layouter.style` (cada palavra e cada nó matemático lê `.size`
  para obter advance horizontal ou vertical_metrics).
- **4 sítios** lendo `.bold`, 3 lendo `.italic`.
- **31 construtores** de `TextStyle` — maioria em tests.

O número alto de reads de `.size` exclui a opção "resolver em cada
leitura" por custo de perfil.

## Decisão

**Estratégia SR (Struct Resolvido)**.

### Forma do `FrameItem::Text.style`

Manter o nome `TextStyle` (evita ripple em ~10 tests legacy) e
**estender** com os campos forward-compat introduzidos no ADR-0038:

```rust
pub struct TextStyle {
    pub bold:          bool,
    pub italic:        bool,
    pub size:          Pt,
    pub fill:          Option<Color>,     // Passo 100 — forward-compat
    pub heading_level: Option<u8>,        // Passo 100 — forward-compat
}
```

Semântica: `TextStyle` é **o resultado** de resolver uma
`StyleChain`, não a cadeia em si. O `From<&StyleChain> for TextStyle`
existente desde Passo 22 é o **ponto único de resolução**.

### Pipeline do Layouter

- `Layouter.style: TextStyle` → substituído por
  `Layouter.chain: StyleChain` como source-of-truth do estilo activo.
- O ponto de emissão do `FrameItem::Text` calcula
  `TextStyle::from(&self.chain)` — é aí que a cadeia é "achatada"
  para o struct resolvido.
- `Content::Styled` activo: o Layouter faz push/pop da cadeia antes
  e depois de layout do body. Substitui o comportamento transparente
  do Passo 99.

Zero lifetime contagion: `StyleChain` usa `Arc` interno (desde Passo
22). Layouter permanece sem parâmetro de lifetime.

### Save/restore pattern

Sítios que hoje fazem:

```rust
let prev = self.style;
self.style = TextStyle::bold(self.font_size_pt);
self.layout_content(body);
self.style = prev;
```

Passam a:

```rust
let prev = self.chain.clone();  // O(1) — Arc::clone
self.chain = self.chain.push_styles(&Styles::from_iter([Style::Bold(true)]));
self.layout_content(body);
self.chain = prev;
```

Paridade de semântica; `StyleChain` é a fonte da verdade.

### `export.rs` em L3

Continua a ler `style.bold`/`.italic`/`.size` directamente. Zero
mudança de contrato — SR mantém a forma resolvida.

### Tests legacy

`TextStyle { bold, italic, size }` ganham `..TextStyle::default()`
automaticamente via transformação Regex. `TextStyle::regular/bold/italic`
constructors preservados.

## Alternativas consideradas

1. **SO (Styles Owned)**: `FrameItem::Text { styles: Styles }`;
   export chama `resolve()`. Rejeitado: o cristalino não tem caso
   de uso para preservar deltas no frame, e resolver em cada read
   de `.size` (55 sítios) adiciona custo sem ganho.

2. **StyleChain owned no FrameItem**: rejeitado (explicitado na
   spec 100.A): lifetimes fariam `FrameItem` não-`'static`,
   quebrando callers.

3. **Renomear `TextStyle` → `Resolved`**: considerado. Rejeitado por
   minimizar ripple nos ~10 tests legacy. O nome `TextStyle` hoje é
   exactamente o que `Resolved` seria — mantendo o nome, o renaming
   é puro noise.

## Relação com outros ADRs

- **ADR-0016** (LazyHash fora de L1) — **preservada**. `StyleChain`
  usa `Arc<StyleNode>`, não `LazyHash`.
- **ADR-0026** (Content enum fechado) — precedente aplicado a
  `Style` (enum linear).
- **ADR-0033** (paridade vanilla) — top-wins preservado (Passo 99).
- **ADR-0036** (atomização) — `StyleChain` já é parâmetro explícito
  nas funções `eval_*` desde o Passo 94; no Layouter passa a ser
  campo de estado interno (não parâmetro) porque o Layouter é uma
  máquina de estado linear, não um pipeline funcional.
- **ADR-0037** (coesão) — novos tests em `layout/tests.rs`
  (cfg(test) gated).
- **ADR-0038** (sistema de estilos em L1) — este ADR fecha o
  circuito ao activar `Content::Styled` no Layouter.

## O que esta ADR não decide

- **Activação de `#set`/`#show` no `eval_markup`**: passo futuro.
  `Content::Styled` é o *contrato* entre eval e Layouter; este ADR
  garante que o Layouter honra o contrato mas não força o eval a
  produzi-lo.
- **Quando `LazyHash` vai para L3**: depende do pipeline incremental
  real.
- **Cache do resultado de `From<&StyleChain>`**: optimização
  diferida. Se perfil mostrar que a resolução é hot-path, abrir
  DEBT dedicado.

## Consequências

### Positivas

- DEBT-48 encerrado; DEBT-1 revisitado (fica provavelmente
  encerrado — a dívida estrutural foi paga).
- Layouter passa a processar `Content::Styled` — a fundação ADR-0038
  fica end-to-end funcional **sem** exigir alteração no eval.
- `TextStyle` unifica com o enum `Style`: cada campo corresponde
  a uma variante.

### Negativas

- Breaking change: os ~31 construtores literais de `TextStyle` em
  tests precisam `..TextStyle::default()`. Transformação mecânica;
  aceitável.
- A resolução `StyleChain → TextStyle` corre uma vez por
  `FrameItem::Text` emitido — no caso comum, chain tem 1-3 nós.
  Não é optimização crítica.

### Neutras

- O nome `TextStyle` preserva-se; semanticamente é agora "resolved
  style" em vez de "primária". Documentação do struct actualizada
  para reflectir a mudança.

---

## Referências

- `00_nucleo/diagnosticos/inventario-textstyle-passo-100.md`
- `00_nucleo/materialization/typst-passo-100.md`
- ADR-0038: ponto de partida.
