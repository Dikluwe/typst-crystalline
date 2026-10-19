# L0 — Passo 1106: Deslocamento Vertical de ~4.95pt Entre Heading e Primeiro Bloco (Secção 32)

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é
investigação.

**Base**: nota externa (2026-08-20). `attach()` confirmado 100% resolvido
(57/57 glifos, `dx=0.00pt` em todos). Novo achado: `dy` constante de
`+4.95pt` do primeiro bloco de equação até o penúltimo (título bate exacto,
`dy=0.00`). Último bloco (`scripts(A)^alpha_beta`) foge do padrão
(`+4.74` a `+5.60pt`, variação de `~0.86pt` entre os 3 glifos, contra
variação de centésimos nos outros blocos).

---

## 1. Hipótese principal — regressão do P1104 no protocolo heading→bloco

**Não é achado novo isolado — é candidato a regressão de um passo já
fechado nesta conversa.** O P1063 validou com precisão exacta 4 casos
isolados (Iso A-D) de heading→bloco, usando um mecanismo que **combinava**
`extra_below` (ajuste directo ao `cursor_y` de saída do heading,
`-6.2224pt` nível 1 / `-3.1209pt` nível 2) **e** o protocolo de colapso
(`prev_block_below_pending`/`block_chain_active`) consumido pelo bloco
seguinte.

O P1104 **removeu `extra_below` por completo** de `heading.rs`, substituindo
por consumo exclusivo via protocolo — mas o P1104 só verificou explicitamente
o caso heading→**parágrafo** (texto), não voltou a rodar os 4 casos
Iso A-D originais do P1063 (heading→**bloco**) com números concretos — só
uma alegação genérica de "cadeia P1057-1103 revalidada e preservada", sem
tabela.

**Hipótese a testar**: se `extra_below` tinha uma função dupla — não só
compensar o gap de colapso (que o protocolo já cobre), mas também corrigir
um overshoot do próprio avanço de linha do heading (`flush_line`) que é
**independente** do valor de `below_pt` — removê-lo por completo pode ter
corrigido o caso parágrafo às custas de quebrar o caso bloco, que já estava
correcto antes.

## 2. Re-rodar os 4 casos Iso A-D do P1063, exactamente como estavam

Não redesenhar — reaproveitar os casos exactos já usados e validados no
P1063:
- Iso A: `Heading 1 → Bloco above: 0pt`
- Iso B: `Heading 2 → Bloco above: 0pt`
- Iso C: `Heading 1 → Bloco above: 0.5em`
- Iso D: `Heading 1 → Bloco above: 2em`

Se algum destes já não bater com Δy=0.0000pt, confirma a hipótese do §1 —
regressão real, não achado novo. Se todos continuarem correctos, a causa é
outra, específica ao contexto da secção 32 (equação de bloco em modo
matemático real, não os blocos genéricos sintéticos do P1063).

## 3. Se confirmado como regressão — desenhar correcção sem repetir o erro do extra_below

Se `extra_below` tinha mesmo dupla função, a correcção não é trazê-lo de
volta (reintroduziria o problema que impedia o parágrafo de funcionar) —
é separar as duas funções: uma correcção de overshoot de linha (se
existir, aplicada sempre, independente do que vem a seguir) e o protocolo
de colapso (`below_pt`/`block_chain_active`, já correcto e não deve ser
tocado). Ler `flush_line()` real do heading para confirmar se há mesmo um
overshoot de avanço de linha não relacionado ao colapso, antes de desenhar
qualquer coisa.

## 4. Investigar o último bloco separadamente, só depois do §1-3 resolvido

`scripts(A)^alpha_beta` combina duas funções nativas (`scripts()`, já
existente; `^`/`_` como sintaxe de attach) — a variação de `~0.86pt` entre
os 3 glifos desse bloco especificamente pode ser:
- Resíduo do mesmo deslocamento geral do §1, só que não perfeitamente
  uniforme por alguma interacção com `scripts()`.
- Um segundo bug independente, específico à combinação `scripts()` +
  sintaxe de attach.

Não investigar isto antes de resolver o deslocamento geral — se a causa
principal for corrigida, medir de novo; o "problema" deste bloco pode
desaparecer ou reduzir-se, tornando mais fácil isolar o que sobra.

## Critérios de verificação

1. Os 4 casos Iso A-D do P1063 — confirmar bater ou não, com números.
2. Se regressão confirmada: causa localizada com código real, não suposição.
3. Deslocamento de `+4.95pt` na secção 32 resolvido, `dy=0.0000pt` do
   primeiro ao penúltimo bloco.
4. Último bloco remedido depois da correcção — variação residual
   caracterizada (desapareceu, reduziu, ou é causa própria a investigar
   à parte).

## Critério de conclusão

- §2 executado com números reais, não presumido.
- Se regressão: causa e correcção separando overshoot de linha (se
  existir) do protocolo de colapso, sem reintroduzir `extra_below`.
- §4 não investigado antes do §1-3 estar fechado.
