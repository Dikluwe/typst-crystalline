# Passo 1058 — Módulos de constantes por domínio, com L0 ligado à proveniência

**Tipo**: Estrutural — não muda comportamento (as constantes já existem, só mudam de
sítio e ganham nome). Zero gate `ADR-0127` esperado, salvo se a extracção revelar um
valor incorrecto pelo caminho (mesmo tratamento de sempre — escalar, não corrigir
inline).
**Motivo**: decisão da conversa anterior — não um ficheiro único de constantes (recriaria
o anti-padrão de hub que o `ADR-0104` e 30+ passos desta frente já combateram), mas
módulos por domínio, seguindo o padrão já validado de `entities/math_constants.rs`.
**Escopo inicial**: `layout/` (domínio mais recente a precisar disto — `par.spacing`,
`par.leading`, `block.spacing`), por ser onde o P1057 acabou de mexer.
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1057.

---

## Fase A — Inventariar as constantes candidatas do domínio `layout`

Reunir, de passos já fechados desta frente, todas as constantes de `layout/` com
proveniência já confirmada:
- `par.spacing = 1.2em` (`par.rs:224`, P1057)
- `par.leading = 0.65em` (`par.rs:210`, já citado em 8 sítios desde o P1054)
- `block.spacing = 1.2em` (`container.rs:342`, P1053/P1054, usado em `equation.rs`,
  `divider.rs` per P1055 — confirmar se é a mesma constante que `par.spacing` ou
  coincidência de valor entre dois mecanismos distintos, não presumir)

Confirmar por `grep` se há mais candidatos em `layout/` não listados aqui.

## Fase B — Criar `01_core/src/compiler/layout/vanilla_defaults.rs`

Cada constante como item nomeado, citação **uma vez**, no ponto de definição:

```rust
/// Espaçamento vertical por defeito entre parágrafos.
/// ref: lab/typst-original/crates/typst-library/src/model/par.rs:224
/// Confirmado por medição directa (P1057): delta de -6.05pt eliminado ao usar este
/// valor em vez de PAR_LEADING para o gap entre parágrafos.
pub const PAR_SPACING: Em = Em(1.2);

/// Espaçamento entre linhas dentro do mesmo parágrafo.
/// ref: lab/typst-original/crates/typst-library/src/model/par.rs:210
pub const PAR_LEADING: Em = Em(0.65);

/// Espaçamento por defeito de blocos genéricos (equation, divider, etc.) quando não
/// participam do fluxo de parágrafo directamente.
/// ref: lab/typst-original/crates/typst-library/src/layout/container.rs:342
/// Confirmar (Fase A) se é mecanismo distinto de PAR_SPACING ou o mesmo valor por
/// coincidência semântica — documentar a conclusão aqui, não deixar ambíguo.
pub const BLOCK_SPACING: Em = Em(1.2);
```

## Fase C — L0 do módulo (`layout/vanilla_defaults.md`)

Um L0 novo, formato já estabelecido, mas com uma secção extra por constante: não só
`file:line`, também a **razão de existir enquanto grandeza separada** (porque é que o
vanilla trata `par.spacing` e `par.leading` como dois valores distintos, não um só) —
isto é o "porquê científico" que motivou este passo, não só o "onde está escrito".

## Fase D — Migrar os usos existentes

Substituir os literais espalhados (8 sítios de `PAR_LEADING`, os usos directos de
`1.2em` em `equation.rs`/`divider.rs`/`rules.rs` do P1057) pelas constantes nomeadas.
**Não** repetir a citação completa em cada uso agora — o nome da constante já
carrega a proveniência (definida uma vez na Fase B); um comentário curto
(`// PAR_LEADING, ver vanilla_defaults.rs`) é suficiente por site, se se quiser manter
rastreável sem reabrir o ficheiro.

## Fase E — Validar

```
crystalline-lint .
cargo test --workspace
```
Zero regressão — é renomeação/reorganização, não mudança de valor.

## Fase F — Confirmar que isto não cria um hub novo

Medir fan-in de `vanilla_defaults.rs` depois da migração — se ultrapassar uma dúzia de
consumidores só dentro de `layout/`, está dentro do esperado (é módulo de domínio, não
hub geral). Se começar a ser importado por `math/`/`stdlib/` também, é sinal de que a
fronteira do domínio estava mal desenhada — parar e reconsiderar antes de continuar a
expandir para outros domínios.

---

## Resultado esperado

`layout/vanilla_defaults.rs` como primeiro módulo de constantes por domínio fora de
`math/`, com L0 a explicar não só a citação mas a razão da grandeza existir separada de
outras. Se este piloto correr bem (Fase F sem sinal de hub), replicar o padrão para
outros domínios (`export/`, `stdlib/text/`) em passos futuros, um de cada vez.
