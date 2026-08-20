# L0 — Passo 1105: Implementar `attach()` como Função Matemática Nativa

**Gate**: `ADR-0127` — nova funcionalidade (não é mudança de comportamento
existente, é ausência de suporte). Ainda assim requer confirmação do dono
por afectar output de qualquer documento que use `attach()`.

**Base**: nota externa (secções 32/35/40, achados de 2026-08-08 e
2026-08-20). `attach(A, ...)`/`attach(sum, ...)` vazam como texto literal
(`"attach(𝐴)"`, `"attach(∑)"`), todos os argumentos nomeados descartados.
`limits()`/`scripts()` já funcionam correctamente — confirmar que são
implementados no mesmo despachante e replicar o padrão, não inventar
mecanismo novo.

---

## 1. Ler o código real antes de tudo

- `01_core/src/compiler/eval/math.rs` (já citado nesta conversa para
  `vec()`, `lr()`, delimitadores — confirmar se `limits()`/`scripts()`
  também vivem aqui).
- Localizar exactamente como `limits()`/`scripts()` são despachados — nome
  da função/match arm que reconhece esses identificadores como funções
  matemáticas nativas, distinguindo-os de identificadores comuns que caem
  em texto literal.

## 2. Confirmar a assinatura real de `attach()` no Typst

Não presumir a assinatura a partir da nota (`attach(A, ...)`) — confirmar
contra a documentação/comportamento real do Typst (`base`, e os cantos
`t`/`b`/`tl`/`bl`/`tr`/`br`, nomes exactos a confirmar) antes de desenhar o
parse dos argumentos nomeados. A nota já indica que "argumentos nomeados
são descartados" — isso é sintoma de que o parser não reconhece `attach`
como caso especial, não que os nomes dos argumentos estejam errados.

## 3. Mecanismo

Replicar o padrão de `limits()`/`scripts()`:
1. Reconhecer `attach` como identificador de função matemática nativa no
   avaliador (`eval/math.rs`), não deixá-lo cair no braço genérico que
   produz texto literal.
2. Extrair `base` (posicional) e os argumentos nomeados de canto.
3. Produzir o nó de `Content` correspondente (provavelmente
   `Content::MathAttach` ou equivalente, já usado por `x^y`/`x_y` per
   `attach.rs`, extensamente tocado nesta investigação P1089-1104) —
   confirmar se `attach()` deve produzir exactamente o mesmo tipo de nó que
   a sintaxe `^`/`_` já produz, ou se precisa de um caminho próprio para os
   4 cantos simultâneos (`tl`/`bl`/`tr`/`br`, que a sintaxe de operador não
   cobre directamente).

## 4. Reaproveitar o mecanismo de layout já extensamente corrigido

Esta investigação (P1089-1104) já corrigiu profundamente o layout de
attach (`glyph_ink_bounds`, `space_after_script`, composição de `MathSize`,
`italics_correction`). Se `attach()` gerar o mesmo tipo de nó que `^`/`_`
já produzem, todas essas correcções já se aplicam automaticamente — **não
reimplementar layout geométrico do zero**, só o parse/avaliação da forma
funcional.

## 5. Medição

Reproduzir os casos da nota (`attach(A, ...)`, `attach(sum, ...)`) e
comparar contra o vanilla — confirmar posicionamento correcto nos 4 cantos
possíveis, não só o caso simples de 1-2 argumentos.

## Critérios de verificação

1. `attach(A, t: x, b: y)` (ou sintaxe real confirmada no §2) produz
   sub/sobrescrito posicionado correctamente, não texto literal.
2. Todos os 4 cantos testados, não só top/bottom.
3. `limits()`/`scripts()` continuam a funcionar — não regressão.
4. Reaproveitamento do layout de attach já corrigido confirmado (§4) — não
   duplicação de lógica geométrica.
5. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Código real de `limits()`/`scripts()` lido e usado como modelo directo.
- Assinatura real de `attach()` confirmada, não presumida da nota.
- Os 4 cantos testados individualmente.
- Nenhuma lógica de layout geométrico duplicada — reaproveitamento do
  mecanismo já corrigido nesta investigação.
