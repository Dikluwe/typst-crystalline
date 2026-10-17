# Testes independentes P1431 — carrier `MathMatrix`
Hash do Código: 4c442c5b

## Dono test-only

Consumer único: `01_core/src/entities/elements/p1431_math_matrix_tests.rs`.

## Obrigação

Construir o carrier pela fronteira pública e discriminar rows/células, lados
opcionais do delimitador, alinhamento, offsets hline/vline e sua ordem, stroke,
gaps relativos e os cinco bits de presença. Clone, igualdade, hash, `repr`,
`plain_text`, `map_content`, `map_text` e reconstruções preservam todos os
campos. `MathAlignPoint` permanece dentro da célula até layout.

P21 compara pares que diferem isoladamente em ordem e conteúdo de células,
lado opcional do delimiter, alinhamento, eixo/ordem de offsets, componente de
stroke inclusive cap, eixo/componente de gap e cada bit de presença. Tanto
igualdade quanto hash devem discriminar cada par.

Contra M17c/M17h, há pares adicionais com presença idêntica e diferença única:
`align:left` versus `align:right`; para cada `row-gap` e `column-gap`, varia-se
separadamente o componente relativo e o absoluto. Tanto `Eq` quanto `Hash`
devem mudar sem apoio discriminatório dos bits de presença.

Zero ou múltiplos carriers, cascata não separável ou extração ilegível são
`Unknown`; nunca contam como sucesso nem como mutante morto.

## Adaptação Memória 5 — proposta pendente ADR-0127

Medição antes da decisão: 00_nucleo/diagnosticos/value-compacto-memoria5-2026-10-08/
before.json.gz e inventory.json congelam este consumer e cada ocorrência
file:line de Value::Stroke. sizes.json mede 272→144 bytes no modelo externo de
Value, sem prova de RSS. O novo carrier público é definido exclusivamente por
00_nucleo/prompts/entities/value.md; esta unidade apenas o consome e não
compartilha ownership. Nenhuma implementação/adaptação antes da confirmação
do manifesto deste pacote.

Responsabilidade desta unidade: Atualizar somente construtores Value de stroke para From<Stroke>; preservar todos os pares independentes de morfologia/presença/hash e expectativas vigentes.

Após confirmação: conservar testes/oráculos vigentes, adicionar o RED→GREEN
pertinente quando houver comportamento de isolamento novo, resselo só do owner
e revalidação. Esta cláusula não amplia semântica, defaults ou fase.
