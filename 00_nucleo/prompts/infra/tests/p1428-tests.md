# Prompt L0 — recalibração pública de observadores `math.frac` P1428

## Owner produtivo

`03_infra/tests/p1428_math_frac.rs` (somente teste L3; nenhum código de produção).

## Entradas congeladas

- passo P1428 SHA-256 `e084331781e0d4ea1ea3204e9b2fcc9db6ee3a90364a8fa646dfe9d242eca93a`;
- manifesto P1428 SHA-256 `0c8c2e4ba45900c52359956df85a745d8c537c3a29c513a75debd0e389cc2a0b`;
- contrato P1428 SHA-256 `d2bce23baccf0677e63a87ca94d3a3b96fd3f0cce7fe567aa1565b585a8ec313`;
- oráculos P1428 SHA-256 `539825153ef99df9c336335e5cb6683d8b373720cf2f8fe75b25d33b999d385a`;
- os quatro L0 de teste P1427 e o teste público P1427 permanecem baseline
  somente leitura; o observador defeituoso de HTML/SVG não é copiado.

Regime: protocolo completo Tekt, executado sem atestação de isolamento. O
autor deste teste exerce os papéis segregados de adversário e testador A/B,
sem ler ou alterar o patch produtivo P1427 e sem autoridade de veredito.

## Obrigação

Materializar P01–P08, C01–C05 e U02–U05 do contrato congelado:

1. extrair texto de HTML atribuível por um único passe sobre os bytes originais;
2. considerar somente text nodes, nunca tags ou atributos;
3. decodificar exatamente uma vez `amp`, `lt`, `gt`, `quot`, `apos` e referências
   numéricas Unicode válidas;
4. devolver `Unknown` para entidade desconhecida/malformada e markup ausente,
   desequilibrado ou ambíguo;
5. avaliar F00/F10/F01/F11 por `eval_to_module_with_sink`, partir do conteúdo do
   `Module`, percorrer canonicamente todo `Content` alcançável e exigir exatamente
   um `MathFracElem`;
6. ler do carrier — nunca do texto da fixture — `line=true`, `style=None` e os
   vetores `00`, `10`, `01`, `11`; zero ou múltiplos carriers são `Unknown`;
7. manter controles atribuíveis das três rotas, N01–N16, constantes OpenType
   96/350 com zeros presentes independentes e isolamento de `binom`;
8. compor a evidência dos bits com o oráculo congelado P1427 de morfologia
   horizontal; nenhum dos dois isoladamente concede P08.

## Poder discriminatório

A suíte e o manifesto de mutações devem rejeitar M01–M12: HTML cru, dupla
decodificação, markup ausente, entidade desconhecida aceite, SVG opaco, carrier
zero/múltiplo aceite, bits trocados/forçados/inferidos, conjunção de layout
removida, controles removidos e crédito concedido a `Unknown`.

`Unknown` é fail-closed: não satisfaz caso obrigatório, não mata mutante e não
aumenta `mutation_score`. O score exigido antes de qualquer selo é `12/12 = 1.0`
nas ordens forward, repetição forward e reverse.

Cada mutação é executável somente neste owner por `P1428_MUTANT=M01` até
`P1428_MUTANT=M12`. Cada ativação ocorre num processo de teste fresco, altera
um único observador/classificador test-only e deve terminar com exit não-zero e
testemunha atribuível. Sem a variável, nenhum hook está ativo e a suíte candidata
deve permanecer verde.

M09 deve sobreviver aos quatro casos de spelling direto e morrer num controle
indireto: `main.typ` contém apenas `#include "case.typ"`, enquanto o carrier
`11` nasce no ficheiro incluído. Assim, inferir dos bytes da fonte principal não
reproduz o carrier. M10 exerce um classificador ternário de conjunção bits+layout;
M11 exerce um meta-gate bitmask que exige simultaneamente C01–C04; M12 exerce
U01 e U06 como `Unknown` e exige crédito zero no score.

O gate de ordem usa um único teste `p1428_runner_ordem_contrato` e a variável
`P1428_ORDER=forward|reverse`. O runner despacha e executa, caso a caso, a
sequência congelada `P01–P08,C01–C05,U02–U05`; `reverse` percorre exatamente o
inverso dessa lista. Cada dispatch executa a asserção correspondente e emite
`P1428_CASE id=<ID> classification=<Preserved|Unknown>`. Ao final, emite o vetor
auditável completo. Forward, repetição forward e reverse devem produzir a mesma
classificação por ID, independentemente da ordem de emissão.

## Aceitação

- `p1428_repr_compara_texto_html_decodificado` prova P01–P03;
- `p1428_decoder_html_e_fail_closed` prova C05 e U02–U03;
- `p1428_modulo_expoe_frac_unica_e_quatro_bits` prova P04–P07 e U04–U05;
- `p1428_controles_rotas_diagnosticos_constantes_e_binom` prova C01–C04;
- `p1428_classificadores_conjuntivo_score_e_meta_gate` prova a conjunção ternária,
  o meta-gate dos quatro controles e a política de score de U01/U06;
- `p1428_runner_ordem_contrato` executa e audita os 17 IDs em ordem forward ou
  reverse exata;
- P08 exige, além do teste de bits, o teste congelado
  `p1427_horizontal_restaura_protecao_causal`; ausência de qualquer conjuntivo
  é `Unknown`, não sucesso;
- qualquer travessia não exaustiva, dependência ausente, timeout, crash ou saída
  ilegível permanece opaca e bloqueia aceitação.

## Limites

Este prompt não autoriza alteração produtiva, edição de artefatos P1427, uso do
SVG como testemunha, inferência de bits pela fonte, selo ou certificado final.
Ele cobre somente a recalibração observável P1428 do fragmento `math.frac`.
