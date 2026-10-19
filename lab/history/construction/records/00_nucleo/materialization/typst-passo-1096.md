# L0 — Passo 1096: Fechar a Largura/Altura de Página Contra o Vanilla Real

**Gate**: `ADR-0127` — mudança de comportamento por defeito (largura de
página em `width: auto`, qualquer documento terminando em attach afectado).

**Base**: causa já diagnosticada no P1090, nunca corrigida — `space_after_script`
trailing (fim de linha com attach) não entra no cálculo de largura de
`layout_equation_measured`. Resíduo medido: `262.240pt` (cristalino) vs
`262.649pt` (vanilla), Δ=`0.409pt`, atribuído a este mecanismo mas nunca
fechado com uma correcção real. Altura de página nunca comparada contra o
vanilla nesta cadeia inteira (P1086-1095 só comparou `oracle` vs
`crystalline`, consistência interna, não paridade externa).

---

## 1. Não presumir que a causa do P1090 é a única

Antes de corrigir só o `space_after_script` trailing, confirmar que não há
mais nenhuma fonte de resíduo de largura — reconfirmar a medição de
`262.240` vs `262.649` **depois** de toda a cadeia P1091-1095 já aplicada
(os números podem ter mudado incidentalmente com as outras correcções; não
assumir que o resíduo continua exactamente `0.409pt`).

## 2. Mecanismo — incluir espaço trailing no cálculo de largura

Ler `01_core/src/compiler/math/layout/mod.rs` (`layout_equation_measured`,
já citado nesta investigação) e `01_core/src/compiler/layout/equation.rs`.
Quando o último item da linha/frame é um resultado de attach (`FrameItem`
proveniente de um nó `MathAttach` com sobrescrito/subscrito à direita), somar
o `space_after_script` correspondente à largura total, em vez de parar no
limite físico do último glifo.

Não presumir que isto é simples de detectar — confirmar como identificar "o
último item veio de um attach" a partir da estrutura de `MathBox`/`FrameItem`
disponível; pode já existir alguma marcação usada por outro mecanismo desta
investigação (P1089-1092 mexeram em `attach.rs` extensivamente) que sirva
para isto sem duplicar lógica.

## 3. Altura de página — nunca verificada contra o vanilla nesta cadeia

Medir a altura real da Secção 30 (`crystalline` pós-toda-a-cadeia) contra o
vanilla genuíno — não `oracle`. A nota original (P1086) já mencionava
diferenças de altura (`182.38pt→184.61pt` para `crystalline`), nunca
reconciliadas com o vanilla directamente. Se houver resíduo, tratar com a
mesma disciplina desta investigação inteira (causa real, não aproximação) —
não presumir que corrigir a largura também resolve a altura.

## 4. Medição

Recompilar a Secção 30 completa (`crystalline` actual, pós-P1095) e comparar
`MediaBox` width/height directamente contra `sec_30_vanilla.pdf` genuíno —
não contra `oracle`, não contra snapshots antigos.

## Critérios de verificação

1. Largura de página: convergir para o valor real do vanilla, com Δ dentro
   da tolerância sub-pixel já usada em toda esta investigação (±0.0005pt),
   não "muito mais perto".
2. Altura de página: mesma tolerância, medida pela primeira vez contra o
   vanilla real nesta cadeia.
3. Re-rodar os 13 pontos de início de linha da Secção 30 (mesmo método do
   P1086/P1095) — zero regressão.
4. `crystalline-lint .` — 0 erros. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- §1: resíduo de largura remedido pós-P1091-1095, não presumido igual ao
  P1090.
- §2: mecanismo de `space_after_script` trailing implementado, código real
  citado.
- §3: altura comparada contra o vanilla genuíno pela primeira vez, não só
  `oracle`.
- Largura e altura convergindo dentro de ±0.0005pt, não "resíduo pequeno".
