# L0 — Passo 1107: Fechar as Duas Pontas do Protocolo de Colapso de Heading

**Gate**: `ADR-0127` — mudança de comportamento por defeito, âmbito amplo
(qualquer heading com espaço antes/depois, qualquer bloco cujo `above`
explícito estava a ser ignorado).

**Base**: P1106 (investigação, causa dupla confirmada com código real):
1. `sequence.rs` reinicia `block_chain_active` sempre que `Content::Space`
   aparece entre heading e bloco (quebra de linha comum), descartando o
   `above` explícito do usuário por completo.
2. `heading.rs` (entrada, lado "acima" do heading) usa 3 constantes
   empíricas (`9.2950`/`3.9160`/`1.6632`) desde o P1063, nunca substituídas
   pelo protocolo genérico — só o lado de saída (`extra_below`) foi
   corrigido no P1104.

---

## 1. Correcção 1 — `Content::Space` na preservação de cadeia, com cautela

**Não adicionar `Content::Space` de forma ampla sem verificar consequências
fora do caso investigado.** `Space` aparece em muitos contextos além de
heading→bloco (dentro de texto corrido normal, entre palavras). Antes de
editar `sequence.rs`:

- Confirmar se `Content::Space` entre **dois elementos de bloco** (heading,
  block, equation, parbreak) é sintacticamente distinguível de espaço
  **dentro de uma linha de texto corrido** — se não for a mesma variante de
  `Content` nos dois casos, adicionar `Content::Space` à lista pode ser
  seguro; se for a mesma variante em ambos os contextos, adicionar
  cegamente pode preservar `block_chain_active` através de texto corrido
  normal, quebrando o caso já corrigido no P1104 (heading→parágrafo).
- Alternativa mais segura, se a distinção acima não existir: preservar a
  cadeia especificamente quando `Content::Space` é seguido do próximo
  elemento real ser um tipo já preservado (`Block`/`Shape`/`Parbreak`/
  `Heading`/`Equation`) — via lookahead no `Sequence` consumer, não uma
  regra cega sobre o tipo do item actual.

## 2. Correcção 2 — eliminar as 3 constantes de entrada

Ler `heading.rs:36-54` (já citado no P1106) por inteiro. Substituir
`9.2950`/`3.9160`/`1.6632` pelo mesmo protocolo genérico já usado no lado de
saída (P1104) e em `block.rs` (P1061): `gap = prev_block_below_pending.max
(above_pt)`, `advance = (gap - prev_block_below_pending).max(0.0)`, com
`above_pt` do heading resolvido como `font_base * above_em` (per a fórmula
já usada na verificação do P1106, `19.80pt` nível 1 / `15.84pt` nível 2 —
os mesmos valores já confirmados no P1063 original).

**Confirmar se o ramo `prev_margin_is_parbreak` (que usava as 3 constantes)
ainda faz sentido separado do ramo `else`, ou se agora colapsa para o mesmo
código** — não presumir a estrutura final sem ver como fica depois de
remover as constantes.

## 3. Critérios de verificação (per proposta já validada)

| Caso | Δ alvo |
|---|---|
| Caso 1: Parágrafo → Heading 1 | 0.0000pt |
| Caso 3: Parágrafo → Heading 2 | 0.0000pt |
| Iso A: Heading 1 → Bloco (above: 0pt) | 0.0000pt |
| Iso B: Heading 2 → Bloco (above: 0pt) | 0.0000pt |
| Iso C: Heading 1 → Bloco (above: 0.5em) | 0.0000pt |
| Iso D: Heading 1 → Bloco (above: 2em) | 0.0000pt |
| Secção 32: Heading → Equação 1 | 0.0000pt |

Todos com tolerância ±0.0005pt, não "reduzido".

## 4. Não-regressão — ampla, dado o âmbito da correcção 1

- Corpus canónico completo (7/7 documentos) — mesma disciplina de medição
  pré/pós já usada em P1061/P1063 (binário antes vs depois, não
  vanilla-vs-cristalino directo, para isolar regressão real de divergência
  pré-existente).
- Re-rodar toda a cadeia P1057-1106.
- Testar explicitamente heading→parágrafo **com** espaço/quebra de linha
  entre eles (o caso comum) — não só os casos sintéticos sem espaço — para
  confirmar que a correcção 1 não regrediu o que o P1104 já tinha corrigido.
- `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1: decisão sobre `Content::Space` justificada com confirmação de
  distinguibilidade sintáctica (ou lookahead), não adição cega.
- §2: as 3 constantes eliminadas, ramo `parbreak` revisto.
- §3: os 7 casos, todos a 0.0000pt.
- §4: corpus 7/7 + heading→parágrafo com espaço explicitamente testado.
