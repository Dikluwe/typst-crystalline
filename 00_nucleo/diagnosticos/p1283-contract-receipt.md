# Recibo independente — C-P1283-v4

**Estado:** `SEALED_SPEC_FOR_IMPLEMENTATION`

Este recibo materializa o contrato observável C-P1283-v4 após o resselo dos
Prompts L0. O estado acima sela as entradas e a especificação para a fase de
implementação; não é certificado da implementação, não afirma equivalência
funcional geral e não afirma que o gate discriminatório já foi executado.

## 1. Papel, regime e sequência causal

- Regime: protocolo completo de materialização segregada, na fase exclusiva de
  autoria do contrato.
- Autoridade exercida: autor do contrato.
- Autoridades não exercidas: implementador, autor dos testes/oráculos,
  adversário e verificador final.
- Predecessor causal: C-P1283-v3 foi invalidado porque o resselo obrigatório
  alterou os campos `Hash do Código` de `sym.md` e `emoji.md`.
- Resultado da releitura v4: apenas esses campos mudaram; as obrigações
  semânticas e a matriz de mutações permaneceram idênticas.

## 2. Entradas congeladas

| Prompt L0 | SHA-256 integral | `Hash do Código` declarado |
|---|---|---|
| `00_nucleo/prompts/compiler/stdlib/sym.md` | `42a2127215e9a37d69c90695e1be66a61320fa086ffd36b51a1ac3441c816f3d` | `9bfa8e7f` |
| `00_nucleo/prompts/compiler/stdlib/emoji.md` | `d5fda4cff41bab53b5e877061a63aeb1fda57896d328552a637e6e9cc8e3291f` | `c4ecf7d3` |
| `00_nucleo/prompts/compiler/stdlib/structural/math.md` | `4441fe00c99c3475179c4a90060f5698e4472a368756ead6e4312781229617bf` | `b761923a` |

Baseline semântico pinado pelos L0:

- vanilla ratificado: `a51e02804`;
- `codex = 0.3.0`, checksum
  `0732ab1a27b4ea05e6f9f60a5122c9924dd5123defde0d8e907f58cf643d40e6`;
- `sym.txt`:
  `6ee467d9939acb5c7d0a3eba30c9f640d157529cbf57df752367deb343e0fc16`;
- `emoji.txt`:
  `8691ca68e09b6fedca61e00e824648e79f7502772eefe7af367e404e26161489`.

Qualquer alteração posterior de um dos três L0, dos seus hashes integrais ou
dos pins de baseline invalida este estado e exige novo resselo.

## 3. Ownership 1:1

| Prompt proprietário | Consumer produtivo único |
|---|---|
| `compiler/stdlib/sym.md` | `01_core/src/compiler/stdlib/sym.rs` |
| `compiler/stdlib/emoji.md` | `01_core/src/compiler/stdlib/emoji.rs` |
| `compiler/stdlib/structural/math.md` | `01_core/src/compiler/stdlib/structural/math.rs` |

Os registos em `eval` são callsites, não consumers proprietários adicionais. O
consumer de `math` pode copiar o scope construído pelo owner de `sym` sem
adquirir co-ownership de `sym`. Os três L0 não declaram Núcleos Tekt; o conjunto
esperado de pins é vazio. V15 e V26 permanecem gates da verificação posterior.

## 4. Contrato observável

Para cada path público selecionado, a relação de paridade é:

```text
(path, existência, kind, valor/default, repr,
 variants e valores, resolução por modifiers,
 diagnóstico observável, comportamento em expressão,
 classificação de extensão)
```

O contrato é no nível da linguagem: semântica, sintaxe e morfologia. Estrutura
de dados Rust, igualdade interna, bytes, ordem física das tabelas e algoritmo
mecânico de construção ficam fora do observável.

## 5. Obrigações fechadas

### `sym`

- 334 símbolos: 297 diretos e 37 sob `gender`/`control`.
- Dois submódulos e 1.206 registos de valor/variante.
- 46 pais sem variante bare.
- Conversão recursiva de módulos, preservando kind, valores integrais, `repr`,
  variants e conjuntos de modifiers.
- VS15, VS16 e sequências ZWJ são parte do valor e não podem ser truncados,
  normalizados ou substituídos.
- Ordem de aplicação dos modifiers é semanticamente irrelevante.
- Para pai sem bare, o default usa o best-match pinado: maior coincidência,
  menos modifiers extra e primeira variante apenas em empate. É proibido
  fabricar bare ou escolher cegamente a primeira variante da fonte.
- Aliases `dollar`/`pataca`, `yen`/`yuan`, `emptyset`/`nothing` e
  `gradient`/`nabla` preservam identidade completa de base e variants.
- Igualdade apenas do valor base não transforma `infinity`/`oo` em alias
  estrutural.
- `sym.sqrt` permanece ausente; markup preserva o grapheme integral.

### `emoji`

- 772 símbolos e 1.386 registos de valor/variante.
- 86 pais sem bare, com o mesmo best-match de `sym`.
- Mesmo kind, valor, `repr`, modifiers e variants do baseline pinado.
- VS15, VS16 e ZWJ são morfologia observável.
- Casos discriminantes mínimos: `emoji.apple` resolve para `🍏`,
  `emoji.arrow` para `↙️`, e `emoji.heart` conserva VS16 e as 22 variants
  especificadas no L0.

### Espelho `sym` para `math`

- O mesmo subgrafo de 334 símbolos, dois submódulos e 1.206 registos é
  observável recursivamente sob `math`.
- A cópia ocorre depois dos bindings próprios de `math` e nunca sobrescreve
  `sqrt`, `class`, `equation` ou `op`.
- Kind, default, `repr`, aliases, modifiers, variants e Unicode integral são
  preservados.
- O escopo de P1283 não inclui materializar outras funções matemáticas
  ausentes.

## 6. Exceções explícitas e `Unknown`

Treze depreciações anexadas diretamente a variants das famílias `gt.tri*`,
`lt.tri*` e `tack*` são classificadas como
`IntentionalVariantDeprecationDivergence`, porque `SymbolVariant` não
transporta mensagem. A ausência desses avisos:

- não viola C-P1283-v4;
- não conta como paridade diagnóstica;
- não autoriza omitir ou alterar símbolo, variant, valor, `repr`, modifiers ou
  resolução;
- não autoriza acrescentar transporte público de mensagem sem novo gate
  ADR-0127.

Depreciação no binding superior continua obrigatória. `join`, inclusive suas
variants e o espelho em `math`, deve resolver e emitir exatamente:

```text
`join` is deprecated, use `bowtie.big` instead
```

`bowtie` não emite esse aviso.

`sym.registered` e `math.registered` são extensões cristalinas preservadas, sem
crédito de paridade e sem remoção automática.

`Unknown` só é permitido para identidade realmente ambígua, fonte opaca, pin
divergente, parser sem suporte ou impossibilidade documentada de observação.
Nunca substitui uma falha confirmada, nunca se confunde com a divergência
intencional e nunca é promovido implicitamente a `Preserved`.

## 7. Matriz de mutações 16+3

As 16 mutações negativas válidas são:

1. remover um símbolo;
2. remover ou achatar um submódulo;
3. trocar o kind;
4. alterar um valor base;
5. remover ou trocar VS15;
6. remover ou trocar VS16;
7. truncar uma sequência ZWJ;
8. remover ou renomear uma variant;
9. alterar o valor de uma variant;
10. tornar a ordem dos modifiers significativa;
11. fabricar bare ou usar a primeira variant indevidamente;
12. quebrar somente um lado de um alias ou suas variants;
13. omitir parte recursiva do espelho sob `math`;
14. sobrescrever um binding próprio de `math`;
15. suprimir ou alterar o aviso do binding `join`;
16. remover `registered` ou creditá-lo como paridade vanilla.

Resultado exigido no gate discriminatório posterior: todas as 16 recebem
`Violated`, com testemunha; `mutation_score = 16/16 = 1.0`.

Três controles, fora do denominador:

1. variant correta sem aviso próprio →
   `IntentionalVariantDeprecationDivergence`;
2. variant ausente sob alegação da exceção → `Violated`;
3. `join` sem o aviso exato → `Violated`.

Este recibo registra a matriz e seus resultados exigidos; o autor do contrato
não executou o gate contra a implementação e não emite o veredito final.

## 8. Declaração de independência e limitações

- Desde o reinício que produziu C-P1283-v2 até este recibo v4, o autor do
  contrato não leu implementação candidata, testes candidatos nem diffs
  produtivos.
- A rodada v4 releu somente os três L0 congelados e escreveu exclusivamente
  este diagnóstico.
- A rodada v1, anterior ao reinício, foi invalidada e não constitui evidência
  deste selo.
- O workspace e o contexto conversacional são compartilhados; portanto, esta
  execução possui segregação procedimental, mas não atestação de isolamento
  técnico por sandbox ou worktree independente.
- Hashes identificam as entradas e a ordem causal, mas não provam por si sós
  isolamento de capacidades.
- Implementação, testes independentes, execução das mutações, V15/V26, testes,
  build, lint e certificado final pertencem às autoridades posteriores.

