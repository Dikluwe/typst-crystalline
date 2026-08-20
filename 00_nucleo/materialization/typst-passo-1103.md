# L0 — Passo 1103: Resetar `last_block_descent_y` Após Texto de Parágrafo

**Gate**: `ADR-0127` — mudança de comportamento por defeito (altura de
página `auto` em qualquer documento com bloco seguido de texto normal).

**Base**: P1102 (investigação). `compute_page_height` usa
`last_block_descent_y` quando disponível, mas esse campo nunca é invalidado
quando texto normal de parágrafo é emitido depois de um bloco — resultando
em usar a posição do bloco anterior (mais alta) em vez da linha de texto
real mais recente (mais baixa), encurtando a página.

---

## 1. Mecanismo

Onde texto normal é emitido (`layout_word`/`push_text`/`flush_line` regular,
per a localização já apontada no P1102 — confirmar o ponto exacto antes de
editar, não presumir qual das três funções é a certa sem ler o código),
resetar `self.last_block_descent_y = None` antes ou durante essa emissão.

**Não presumir que "resetar sempre que há texto" é suficiente** — confirmar
se há casos onde `last_block_descent_y` deve persistir através de texto
(ex.: texto dentro do mesmo parágrafo que só continua depois do bloco sem
nova linha) vs casos onde deve mesmo ser descartado. Ler o uso completo de
`last_block_descent_y` nesta e nas funções relacionadas antes de decidir
onde exactamente resetar.

## 2. Verificar se este campo tem mais consumidores

`last_block_descent_y` pode ser lido por mais do que só
`compute_page_height` — confirmar (`grep -rn "last_block_descent_y"`) antes
de mudar o seu ciclo de vida, para não quebrar outro consumidor que dependa
do comportamento actual (mesmo que esse comportamento actual esteja errado
para `compute_page_height`, pode estar certo para outra coisa).

## 3. Medição

- `MediaBox Height` da secção 31: convergir para `263.8196pt` (valor
  vanilla), não "mais perto".
- Testar também o caso inverso — bloco como **último** elemento da página
  (sem texto depois) — confirmar que `last_block_descent_y` continua a ser
  usado correctamente nesse caso (não regressão do caso que já funcionava,
  presumivelmente a razão original deste campo existir).
- Testar múltiplos blocos consecutivos, sem texto entre eles — confirmar
  que o reset não interfere com esse caminho (não há texto para disparar o
  reset, o campo deve continuar a actualizar normalmente bloco a bloco).

## 4. Não-regressão

Re-rodar P1086-1102 (largura já corrigida no P1102) — zero regressão.

## Critérios de verificação

1. `MediaBox Height` da secção 31 convergindo (±0.0005pt).
2. Caso "bloco é o último elemento" — sem regressão.
3. Caso "múltiplos blocos consecutivos sem texto entre eles" — sem
   regressão.
4. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Ponto exacto de reset confirmado no código real, não presumido.
- §2 respondido — outros consumidores de `last_block_descent_y`
  identificados e considerados.
- Os 2 casos de não-regressão do §3 testados, não só o caso que motivou a
  correcção.
