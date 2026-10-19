# L0 — Passo 1122: Secções 18, 19, 20, 23 — Generalização do Espaçamento Entre Blocos e um Achado Novo

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é
investigação. Prioridade: secção 19 (isola a causa com mais clareza) →
secção 23 (tem sintoma novo, não só o já conhecido) → secção 20 (confirmar
se já devia estar coberta por correcção anterior) → secção 18 (menor,
catalogar).

---

## 1. Secção 19 — `display()`/`script()`/`sscript()` acumulam espaço; `#h()` não

**Achado central, já bem isolado pela nota**: `#h()` manual está perfeito
(`dx=dy=0.00` nas 5 primeiras linhas); só as 4 últimas (`display`/`inline`/
`script`/`sscript`) acumulam, crescendo a cada bloco (`-0.45→+6.42→
+8.77-13.57→+15.35-17.99pt`).

**Não presumir que é a mesma causa já corrigida em P1115** (leading de
parágrafo a inflar `descent` em sub-frame de `text(size:)`) — confirmar se
`display()`/`script()`/`sscript()` passam pelo mesmo caminho de código
(`layout_external`) que `text(size:)` já corrigido, ou se têm um caminho
próprio de mudança de `MathSize`/contexto que nunca foi tocado por essa
correcção. Ler o código real antes de assumir que a correcção do P1115
devia ter coberto isto e não cobriu.

**Verificação**: as 4 linhas (`display`/`inline`/`script`/`sscript`)
convergindo para `0.0000pt`, mesma disciplina de sempre.

## 2. Secção 23 — sintoma novo: oscilação horizontal, não só crescimento vertical

**Não é só o padrão já conhecido.** A primeira equação (`Res(...)`) tem
`dy≈0.00` mas `dx` a oscilar `±1.83pt` "indo e voltando conforme o termo"
— isto é diferente do padrão de crescimento monótono já visto (P1090-1097,
P1108). Investigar separadamente antes de presumir que é a mesma família.

**Hipótese a testar**: operadores personalizados (`Res`, `Var`, `Cov` —
nomes de função matemática definidos pelo usuário, não nativos) podem usar
um caminho de medição de largura diferente do das funções nativas já
corrigidas nesta investigação (`sum`, `integral`, etc.) — se a extensão de
cada termo dentro do operador for medida de forma inconsistente
(nominal vs real, mesma classe de bug já vista em P1098/P1101), isso
explicaria oscilação em vez de deslocamento constante.

A segunda e terceira equações (`Var`, `Cov`) têm o padrão de crescimento já
conhecido (`1.62pt`, `3.54pt`, incremento `~1.6-1.9pt`) — **isto pode ser a
mesma causa da secção 19/20**, não presumir isolado até confirmar.

## 3. Secção 20 — confirmar se já devia estar coberta pela correcção do P1108

A nota trata isto como "sem novidade", mas o valor (`~4.92-4.95pt`,
constante a partir do primeiro bloco) é **quase idêntico** ao que a secção
32 tinha **antes** do P1108 corrigir a transição `Heading→Equação` (que
media exactamente `+4.9492pt`, per P1108). Se essa correcção foi mesmo
genérica (via o mesmo protocolo `block_chain_active` já unificado em
P1107), a secção 20 devia já estar resolvida — reaparecer com a mesma
magnitude é suspeito, não "sem novidade".

**Não aceitar a explicação da nota sem verificar.** Hipótese a testar:
a secção 20 envolve **numeração de equação** (referências cruzadas,
per o título da secção) — se a correcção do P1108 só cobriu o caminho
comum de `equation.rs` e a numeração usa um caminho de código diferente
(ex.: outro ponto de entrada para equações numeradas vs não numeradas),
a correcção pode não ter generalizado para este caso.

**Verificação**: confirmar se a secção 20 usa `#set math.equation(
numbering: ...)` (a nota menciona "Referências Cruzadas e Numeração" no
título) — se sim, testar isoladamente se equações **sem** numeração no
mesmo tipo de contexto (heading + bloco) já convergem (confirmando que o
problema é específico ao caminho de numeração) ou se ainda falha (a
correcção do P1108 não generalizou de todo).

## 4. Secção 18 — catalogar, menor prioridade

Três resíduos pequenos, sem urgência:
- Fração contínua: `dx=0.5-2.9pt` por nível de profundidade — possível
  acumulação por nível de aninhamento, investigar só depois das frentes
  acima.
- `√a+√b+√c`: `dy=-1.63 a -3.56pt`, linha inteira desviada — pode ser
  mesma família de "descent`/`ascent` mal medido" já vista nesta
  investigação (P1104, `radical_extra_ascender`) mas para caso diferente
  (raiz sem índice, múltiplas raízes na mesma linha).
- `underbrace(overbrace(...))`: pequeno, `dx=-1.7 a 2.9pt`, `dy≈0`.

## Critérios de verificação

1. Secção 19: as 4 linhas convergindo, causa identificada (mesma ou
   diferente do P1115).
2. Secção 23: oscilação horizontal da primeira equação investigada à
   parte; segunda/terceira equações relacionadas ou não à mesma causa da
   secção 19/20, confirmado.
3. Secção 20: confirmado se é regressão/lacuna da correcção do P1108
   (não coberto por numeração) ou causa nova.
4. Secção 18: catalogado, não necessariamente corrigido neste passo.
5. Re-rodar P1086-1121 — zero regressão.
6. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Secção 19 e 23 com código real citado, causas próprias confirmadas.
- Secção 20 não aceita como "sem novidade" sem verificação — resolvido se
  o P1108 já cobria e algo quebrou, ou identificado como lacuna nova
  (caminho de numeração nunca coberto).
- Secção 18 pelo menos catalogada com hipótese, mesmo que não corrigida
  neste passo.
