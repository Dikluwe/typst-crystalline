# L0 — Passo 1130: Espaço Barra→Radicando Encolhido em `sqrt()`/`root()`

**Gate**: `ADR-0127` se confirmar necessidade de correcção. Este passo é
investigação.

**Base**: nota externa (secção 1). Símbolo da raiz quase não se move
(`+0.26pt`); o **radicando** sobe em relação à barra — espaço
barra→radicando encolhe `-1.18pt` (`sqrt`) e `-2.19pt` (`root` cúbica,
quase o dobro). Na raiz cúbica, isto desalinha o índice `3` do radicando
(`0.15pt` de diferença no vanilla, `2.17pt` no oracle). **Possível
regressão** de um achado antigo ("altura do radical não escalando com o
radicando") — não confirmada, sem o relatório original disponível nesta
conversa.

---

## 0. Não presumir relação com o achado antigo sem o documento real

Não tenho o `.md` original citado pela nota. Se for possível obtê-lo,
comparar directamente o mecanismo (lá era sobre **escala da barra**; aqui
é sobre **posição vertical do radicando relativo à barra**, per a
própria nota — podem ser sintomas diferentes de causas diferentes,
apesar de ambos serem "geometria do radical"). Sem o documento, tratar
como achado novo, independente, até prova em contrário — não presumir
"voltou igual" nem "é completamente diferente" sem evidência.

## 1. Ler o código real antes de investigar

`root.rs` (já referenciado nesta investigação, P906/P1104 —
`radical_extra_ascender`, `descent_surd`) — confirmar o mecanismo actual
de posicionamento vertical do radicando dentro da barra, distinto do
mecanismo de escala/altura da própria barra (que pode já estar correcto,
per "o símbolo da raiz quase não se move").

## 2. Testar mais casos, per a recomendação da própria nota

- Radicando com fracção dentro (`sqrt(a/b)`).
- Radicando com sobrescrito (`sqrt(x^2)`).
- Confirmar se o encolhimento do espaço barra→radicando é proporcional
  ao "peso"/altura do radicando (cresce com radicando mais alto,
  consistente com "quase dobra de `sqrt` simples para `root` cúbica") ou
  se é um valor mais ou menos fixo que só parece crescer por
  coincidência destes dois casos específicos.

## 3. Desalinhamento do índice em `root()` cúbica — causa própria ou consequência?

Confirmar se o índice (`3`) desalinha **porque** o radicando subiu (o
índice fica posicionado relativo à barra, que não se moveu, enquanto o
radicando subiu, criando a distância aparente) — nesse caso, corrigir a
posição do radicando resolve os dois sintomas de uma vez, sem tocar no
posicionamento do índice em si. Não presumir sem confirmar.

## Critérios de verificação

1. Espaço barra→radicando convergindo para `2.02pt` (`sqrt`) e `8.51pt`
   (`root` cúbica), não "reduzido".
2. Índice `3` da raiz cúbica realinhado com o radicando (`~0.15pt` de
   diferença, não `2.17pt`) — confirmar se resolvido como cascata do §3.
3. §2: pelo menos 2 casos adicionais testados (fracção, sobrescrito no
   radicando).
4. Re-rodar P1086-1129 — zero regressão.
5. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §0: relação com achado antigo esclarecida se o documento for
  disponibilizado; tratado como independente caso contrário.
- §3: desalinhamento do índice tratado como cascata ou causa própria,
  confirmado com dados, não presumido.
