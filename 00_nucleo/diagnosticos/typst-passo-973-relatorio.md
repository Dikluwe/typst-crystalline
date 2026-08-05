# Relatório — Passo 973 (varredura de catch-alls sobre `FrameItem` em `math/layout/`)

**Data:** 2026-08-05
**Proveniência**: HEAD = `f4385d29d` (P970-parte-2 + P971), árvore limpa no
início. **Nenhum código de produção alterado neste passo** — o resultado
da varredura é negativo (não há mais bugs deste tipo em escopo), logo não
há benchmark/comparador novo a correr: o binário é idêntico ao de P971
(a única escrita em ficheiros L1 foi o resselo de hashes em comentários de
header). Suíte corrida no fim por disciplina: 9 crates ok, **5742 testes,
0 falhas**.

## Fase A — inventário completo

Todos os `match`/`if let`/`filter_map`/`retain` sobre `FrameItem` em
`01_core/src/engine/math/layout/` (produção; testes excluídos):

| # | Sítio | Forma | Veredicto |
|---|---|---|---|
| 1 | `mod.rs:67` (`MathBox::place`) | exaustivo, 8 variantes; no-ops documentados | legítimo |
| 2 | `mod.rs:119` (`offset_item`) | exaustivo, 8 variantes | legítimo |
| 3 | `mod.rs:487` (extent de `layout_equation_measured`) | exaustivo; braço agrupado documentado | legítimo (ver nota) |
| 4 | `mod.rs:1067` (`hconcat_spaced`) | exaustivo; no-ops documentados | legítimo |
| 5 | `frac.rs` (ciclos num/den) | **era o bug de P972** — já corrigido (usa `offset_item`) | corrigido |
| 6 | `attach.rs:246` (find_map da IC, P971) | helper de procura, `_ => None` correcto | legítimo |
| 7 | `matrix.rs:213` (split de células) | match sobre **`Content`**, não `FrameItem` | fora do alvo |

Notas de detalhe: no #3, o braço `Glyph` usa aproximação por cap-height
(documentada, P813) — aproximação conhecida, não omissão. Os braços no-op
("Image/Shape/Group/Link não ocorrem em contexto math") foram verificados
contra os construtores do módulo: math só constrói `Text`, `TextShaped`,
`Glyph` e `Line` (assembly/cancel/root/stretchy/delimited). Nenhum
`filter_map`/`if let`/`retain` sobre `FrameItem` existe no módulo
(greps vazios).

**Fase A.1 (priorização)**: zero bugs novos confirmados. O único braço
defeituoso da classe era o de P972, já corrigido — a dúvida que motivou o
passo ("há mais?") fica respondida: **não, em `math/layout/`**.

**Suspeito fora de escopo** (registado para passo futuro, não corrigido):
`01_core/src/engine/layout/sub_frame.rs:306` — a altura de um sub-frame só
considera `Text`/`TextShaped` (`_ => {}` para o resto, fallback `line_h`).
Conteúdo math (Glyph/Line) num sub-frame ficaria subestimado. Mesmo
padrão, fora do directório-alvo; precisa de reprodução própria.

## Fase B — nada a corrigir

Sem bugs novos confirmados, não houve TDD neste passo. O teste de
integração com fonte real que "teria pego" a classe de bug já existe —
são os `p972_tests` (numerador/denominador com `Glyph`), escritos em P972.

## Fase C — prevenção (decisão registada em `_comum.md` §P973)

Convenção adoptada (L0 editado, resselo feito):

1. Matches sobre `FrameItem` em `math/layout/` listam as 8 variantes —
   nunca `_ =>`; a exaustividade fica verificada pelo compilador.
2. Braços no-op intencionais mantêm comentário com a razão.
3. Translações sempre via `offset_item` — nunca matches locais novos.
4. **Lint dedicado não implementado** — custo desproporcionado (6 sítios,
   todos correctos hoje; a convenção + revisão bastam). Reavaliar se o
   padrão voltar a falhar. (`#[non_exhaustive]` foi considerado e
   rejeitado: forçaria braços `_` em todo o lado — o efeito oposto ao
   pretendido.)

**Linter**: `crystalline-lint .` → 0 violations (só o V7 órfão
pré-existente).

## Resultado

- Inventário completo com veredicto por sítio (tabela acima): a classe de
  bug de P972 não tem mais instâncias em `math/layout/`.
- Convenção de prevenção registada no L0; lint estrutural decidido
  (não implementar, com a razão registada).
- Um suspeito fora de escopo (`sub_frame.rs:306`) catalogado com
  `file:line` para passo futuro.
- Suíte verde; binário inalterado; linter limpo.
