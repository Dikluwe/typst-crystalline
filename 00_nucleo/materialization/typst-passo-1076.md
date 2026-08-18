# L0 — Passo 1076: `color.mix` em sRGB Difere 1 Unidade no Canal Verde — Achado #11 do P1031

**Gate**: `ADR-0127` — mudança de comportamento por defeito, prioridade Baixa
(diferença de 1/255 num canal, quase imperceptível visualmente, mas ainda assim
divergência de paridade). **Requer confirmação do dono antes de codificar.**

**Base**: P1031, Achado #11 (citação literal confirmada no P1071): vanilla
`#805a88`, cristalino `#805b88` — diferença de 1 no canal verde (`5a`=90 vs
`5b`=91). Código citado no P1071: `01_core/src/entities/color.rs:671-678`,
"Interpolação linear de componentes em `mix()` com conversão `to_srgb()`".

---

## 1. Ler antes de codificar

Só tenho a descrição em prosa do P1071, não o código real. Pedir:

- `00_nucleo/prompts/entities/color.md`
- `01_core/src/entities/color.rs`

## 2. Hipótese a confirmar, não assumir — onde entra o arredondamento

Uma diferença de exactamente 1 unidade (não uma divergência grande) é a
assinatura típica de três causas possíveis, não intercambiáveis:

1. **Arredondamento diferente no `f64 → u8` final** — `.round()` (arredonda ao
   mais próximo) vs truncar (`as u8`, trunca para baixo) vs `.round_ties_even()`
   — cada um pode dar resultado diferente no limiar exacto de `.5`.
2. **Ordem das operações** — interpolar em espaço linear (gamma-decodificado)
   depois converter para sRGB, vs interpolar directamente nos componentes sRGB
   sem decodificar — isto produziria diferenças maiores que 1 unidade em geral,
   mas vale confirmar antes de descartar.
3. **Acumulação de precisão de ponto flutuante** — ordem de multiplicação/soma
   diferente entre os dois binários pode produzir `89.996` vs `90.004`, que
   arredondam para lados opostos de `90`.

**Não presumir qual das três é a causa** — confirmar contra o código real do
vanilla (`lab/typst-original/crates/typst-library/src/visualize/color.rs`,
função `mix` ou equivalente) e do cristalino, comparando a fórmula passo a
passo, não só o resultado final.

## 3. Medição

Reproduzir o caso exacto do P1031 (`#805a88`/`#805b88`) e, se possível, variar
os inputs de `mix()` para confirmar se a diferença de 1 unidade é sistemática
(sempre no mesmo sentido, sinal de arredondamento consistente) ou aparece só
nalguns valores (sinal de acumulação de precisão dependente do caso).

## 4. Mecanismo (condicional à Parte 2)

Não desenhar a correcção antes de confirmar a causa real. Se for
arredondamento, a correcção é trivial (trocar o método de arredondamento para
bater com o vanilla). Se for ordem de operações ou precisão, pode exigir
reestruturar a função `mix()` inteira — âmbito bem maior, merece confirmação
extra com o dono antes de prosseguir mesmo depois deste L0 já ter sido
homologado para o caso simples.

## 5. Critérios de verificação

1. Caso do P1031 — `#805a88` exacto, não `#805b88`.
2. Pelo menos 2-3 casos adicionais de `mix()` com proporções diferentes
   (25%/75%, 50/50, etc.) — confirmar que a correcção generaliza, não é
   ajustada só para bater com o caso único já conhecido.
3. `crystalline-lint .` — 0 erros.
4. `cargo test --workspace` — 100% pass.

## Critério de conclusão

- Código real lido (§1).
- Causa identificada com prova (§2), não suposição.
- Correcção proporcional ao âmbito real da causa (§4) — não maior nem menor do
  que o necessário.
- Critérios de verificação confirmados com mais de um caso.
