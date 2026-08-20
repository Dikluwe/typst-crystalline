# L0 — Passo 1101: Corrigir `limits` Padrão em Modo Inline (Secção 31)

**Gate**: `ADR-0127` — mudança de comportamento por defeito (qualquer
operador com limites, `sum`/`prod`/`integral`/etc., usado inline `$...$` é
afectado).

**Base**: P1098 (diagnóstico, não implementação) — identificou que
`math/layout/mod.rs` empilha limites verticalmente mesmo em modo inline
(`MathSize::Text`), quando o vanilla usa scripts laterais por defeito nesse
modo, reservando o empilhamento para `MathSize::Display`. Isto foi apontado
como a causa mais provável do sintoma de margem da secção 31 (via inflação
de geometria da linha), mas a relação nunca foi confirmada numericamente —
só hipotetizada.

---

## 1. Ler o código real antes de corrigir

`01_core/src/compiler/math/layout/mod.rs` — localizar onde `limits`
(booleano ou enum) é decidido por defeito para operadores com
sub/sobrescrito (`sum`, `prod`, `integral`, `lim`, etc.). Confirmar o nome
real do campo/função antes de editar — não presumir a partir da descrição
do P1098.

## 2. Mecanismo

Alterar o default: `limits: true` (empilhado) só quando
`style.math_size == MathSize::Display`; `limits: false` (scripts laterais)
para `MathSize::Text` (inline) e `MathSize::Script`/`ScriptScript` (se
aplicável — confirmar contra o vanilla se scripts aninhados também usam
scripts laterais, não presumir só os dois casos já mencionados).

Não alterar o comportamento quando `limits: true`/`false` é definido
explicitamente pelo usuário (`op(limits: true)` ou equivalente) — isto é só
sobre o **default**.

## 3. Verificar se isto sozinho resolve a margem da secção 31

Voltar ao caso da nota original (P1086, secção 31): `sum_(k=1)^n k^2`
inline, dentro de parágrafo. Medir margem direita/inferior da página
`width/height: auto` antes e depois desta correcção:

- Se a margem convergir para o esperado (`28.35pt`/`26.21pt`, per a nota
  original) só com esta correcção → confirma que a causa 1 (empilhamento
  indevido) era suficiente, a segunda causa do P1098 (`pending_equation_
  centering` sobre página já inflada) era efeito em cascata, não bug
  próprio — mesmo padrão de raciocínio já confirmado no P1100 para
  `block_chain_active`.
- Se não convergir totalmente → a segunda causa do P1098 precisa de
  correcção própria, não presumir que "cascata" se aplica aqui sem medir.

## 4. Medir o offset secundário de ~1.27pt por linha

O P1098 mencionou este offset como "bem menor, mas consistente" sem
confirmar a relação com a causa principal. Depois desta correcção, verificar
se desaparece também (mesma cascata) ou persiste como causa independente.

## 5. Não-regressão

- Re-rodar toda a cadeia P1086-1100 — `limits` já foi tocado indirectamente
  por outros passos desta investigação (attach.rs, etc.); confirmar zero
  regressão nos casos já fechados.
- Testar operadores com limites em modo **bloco** (`$ sum_(k=1)^n k^2 $`,
  com espaços) — devem continuar empilhados, não regressão do caso Display.

## Critérios de verificação

1. Margem direita/inferior da secção 31 convergindo para o valor do vanilla
   (±0.0005pt), não "muito mais perto".
2. Offset de linha (~1.27pt) resolvido ou identificado como causa
   independente, com dados.
3. Operadores com limites em modo bloco continuam empilhados (não
   regressão).
4. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass
   (confirmar contagem real, 5.955 ou o número actualizado, não presumir).

## Critério de conclusão

- Código real de decisão de `limits` lido e citado.
- §3 respondido com medição real — cascata confirmada ou segunda causa
  identificada.
- §4 respondido — offset de linha explicado, não deixado como "menor, mas
  sem causa".
