# Representação de medidas: arredondamento finito e linhagem compartilhada

Corrigido o arredondamento interno usado por repr. Medidas finitas grandes
em em, graus e porcentagem deixam de virar infinito durante a formatação.
As assinaturas, casas decimais usuais e política de formas não finitas
permanecem. Nenhum aviso foi suprimido.

## Medição anterior à decisão

Branch Tekt, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `representacao-medidas-ci-2026-10-13/before.json.gz`
registra fontes, owners, referências, UTC, estado Git e diffstat.
`probe-before.json`, arquivado em `evidence.json.gz`, registra comandos,
hashes dos binários e resultados completos. O oráculo é o build comprovado
do pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

A fórmula anterior em `01_core/src/compiler/eval/repr/measure.rs:72-78`
multiplicava por uma potência decimal sem proteger a mantissa preenchida.
Na amostra, seis representações divergiam e seis reavaliações de repr
perdiam o valor. A fonte upstream `typst-utils/src/round.rs:26-60` evita
o overflow intermediário e trata os limites de precisão antes da escala.

O controle `1e307pt / 1pt` já era não finito no vanilla e finito no Tekt,
antes desta alteração. A construção upstream em `layout/abs.rs:32` usa
escala interna de 127 por ponto (`:265-273`) e transborda. Por isso, a
igualdade anterior de `repr(1e307pt)` como infinito mascarava duas causas
diferentes. A correção agora preserva o valor finito recebido pelo
formatador Tekt; não declara paridade desse caso de construção.

## L0 e implementação

Os L0 foram escritos antes dos testes e da implementação. O novo núcleo
`00_nucleo/prompts/_nuclei/decimal-rounding.toml` compartilha apenas as
regras finitas entre os owners de repr/measure e calc/round. Ambos o
consomem por SHA-256 completo, mantendo ownership 1:1. O núcleo não é
alvo direto de código. O corpo Rust de calc/round permaneceu idêntico;
apenas seu header de linhagem foi resselado.

O helper de repr preserva valores não finitos, valores integrais grandes
com precisão não negativa e precisões acima dos dígitos úteis de f64.
Precisões inferiores a -308 produzem zero com sinal. Nos demais casos,
libm::exp10 calcula o fator: precisão positiva multiplica antes de
arredondar e divide depois; negativa divide antes e multiplica depois.
Os limites são tratados antes de abs sobre i32, incluindo i32::MIN.
O empate permanece away from zero. Overflow real continua produzindo a
forma de infinito existente; a política de erro específica de calc não
foi transplantada para repr. Libm já estava fixada em 0.2.11 e autorizada.

## Correção da metadata de consumo

A primeira rodada passou nos testes/build, mas o linter pinado do CI
rejeitou o novo núcleo como órfão. O cabeçalho usado inicialmente não era
o campo canônico `Núcleos Tekt:`. Ele foi substituído nos dois owners,
o pin foi corrigido para o SHA-256 efetivo do núcleo (com domínio Tekt,
não apenas o hash bruto do arquivo), os hashes foram resselados e a rodada
foi repetida. O reparador restrito recusou a alteração implícita do pin;
a correção explícita segue a função `effective_nucleus_hash` da fonte
pinada, preservada no recibo. A falha original e
os recibos estão em `initial-validation/`; `metadata-correction.json.gz`
comprova que nenhum corpo Rust mudou nessa correção documental. O pin
do linter permaneceu inalterado. A rodada seguinte passou na linhagem; o diff check apontou somente uma
linha em branco ao fim do L0 de calc. Ela foi removida, com resselo, build,
linhagem, formatação e diff check repetidos. `reference-correction.json.gz`
comprova que essa limpeza também preservou os corpos Rust. As aprovações
abaixo usam esses recibos finais; os testes e Clippy foram medidos antes
da limpeza exclusivamente documental.

## Regressões e linguagem

Quatro testes novos cobrem representação finita com unidade e componente,
precisões extremas, escala grosseira, empates, zero com sinal e não finitos.
Antes da implementação, três falharam e dois passaram no módulo (inclui o
teste anterior de duração). A escala -308 revelava perda de precisão pelo
fator subnormal: 1.0000000000000006e308 em vez de 1e308. Não era apenas um
caso de resultado infinito. `red-source.json.gz` e `red.log` preservam o
estado e as falhas. Depois, todos os testes do módulo passaram.

O script reprodutível `probe.py` compara linguagem, não igualdade de
estruturas Rust. As 16 representações amostradas coincidem com o oráculo
após a correção; antes, seis divergiam. As sete reavaliações preservam
tipo e valor nos dois binários; antes, seis falhavam no cristalino.
Incluem sinais opostos e o controle em 2^53. Valores usuais, zero e
infinitos conservaram suas representações. `float.nan * 1em` resulta em
0pt por comportamento anterior à formatação; a política NaN do formatador
é coberta diretamente no teste, sem inferi-la dessa expressão.

## Validação

Rodada principal concluída em `2026-10-14T00:33:08.342115+00:00`; rustdoc concluído em
`2026-10-14T00:34:01.199272+00:00`. `pre-validation.json.gz` delimita a rodada funcional;
`reference-correction.json.gz` registra a limpeza documental final;
`after.json.gz` registra o estado final e `evidence.json.gz` reúne logs,
receitas, diffs e sondas, com hashes verificados após descompressão.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes, -D warnings | 49 → 48 diagnósticos distintos; ainda falha |
| disallowed_methods no core lib/testes | zero |
| Check de todos os targets do workspace | passou |
| Suíte core | 6.709 passaram, zero falhas, cinco ignorados |
| Nomes de testes | 6.710 → 6.714; quatro novos, nenhum removido |
| Build, formatação e diff check | passaram |
| Rustdoc workspace, -D warnings | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter arquitetural completo | zero erros, 380 avisos, 1611 notas |

A CI completa ainda não está verde. Restam diagnósticos de Clippy e as
validações remotas da matriz de release. Este lote não certifica todos os
valores da linguagem nem corrige a divergência de construção em pt ou as
outras pendências funcionais da auditoria. Nenhuma release foi publicada.
