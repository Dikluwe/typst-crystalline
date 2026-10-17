# Arredondamento finito e precisão inteira

Corrigidos overflow intermediário e truncamento modular de `digits` em
`calc.round`. O lote altera um consumer, seu owner L0 e a autorização da
função pura `libm::exp10` já disponível na dependência pinada 0.2.11.

## Medição antes da decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. O diretório `arredondamento-ci-2026-10-13/` conserva em
`before.json.gz` o horário UTC, status, diffstat e conteúdo/hash das fontes,
L0, configuração e referências somente de leitura. `red.json.gz` conserva
o teste, o L0 e a falha anterior à implementação. Os logs, sondas e receitas
estão reunidos em `evidence.json.gz`.

Na fonte anterior, `01_core/src/compiler/stdlib/calc/round.rs:67-69`
convertia precisão i64 para i32 por truncamento; linhas 117-119 usavam
potência com expoente negativo e multiplicação antes do arredondamento.
As referências medidas são `typst-utils/src/round.rs:26-60` e
`typst-library/src/foundations/calc.rs:923-949` da árvore local em `lab/typst-original`.

O oráculo é o pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore ratificada
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita no diagnóstico P1489,
binário `lab/typst-original/target/release/typst`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O script `probe.py` recusa um oráculo com outro hash. Cada rodada registra
comandos, stdout, stderr, exit code, UTC, estado Git e hashes dos binários.

Foram executadas 60 expressões por binário, incluindo 24 verificações de
tipo e igualdade de valor na linguagem. Antes da correção, todas essas 24
passavam no oráculo e 15 falhavam no Tekt. Exemplos: `1e308` com `digits: 1`
retornava erro por infinito intermediário; `1.4e308` com `digits: -308`
retornava valor diferente de `1e308`; `1.25` com precisão `4294967296`
era arredondado a `1.0`. Precisões extremas afetavam também Int e Decimal.

O ensaio inicial usava o literal negativo mínimo i64, que o parser do
oráculo rejeita. Ele foi substituído, antes da implementação, por
`(-9223372036854775807 - 1)`. A baseline válida é
`probe-before-refined.json`; o primeiro ensaio permanece arquivado e não
fundamenta as conclusões de arredondamento.

## Decisão e implementação

O L0 foi atualizado antes dos testes e da implementação, em fluxo contínuo
ADR-0127: fórmula interna e paridade, sem nova API, flag ou fase. A precisão
permanece i64 até ser saturada a i16 para Int/Float ou i32 para Decimal.
Valores Float finitos de módulo >= 2^53 dispensam arredondamento com
precisão positiva. Precisão negativa usa divisão por uma potência positiva
antes de arredondar, evitando o fator subnormal em -308.

Empates continuam away from zero; precisão suficientemente negativa mantém
o zero com sinal. `guard_float` continua no mesmo regime de precisão,
rejeitando NaN, infinito e overflow real nesse regime. A saturação corrige
também a seleção de regime em precisões que antes sofriam truncamento.
Floor, ceil, trunc, fract e as mensagens de validação não mudaram.

Os quatro testes novos chamam a função nativa com argumentos nomeados.
Todos falharam antes da implementação; depois passaram. Não usam uma cópia
da fórmula como oráculo. Nenhum teste existente foi removido.

## Validação

A rodada principal encerrou em `2026-10-13T23:59:14.661482+00:00`.
`pre-validation.json.gz` delimita as fontes dessa rodada, que permaneceram
inalteradas durante os comandos. Após ela, foi corrigido somente um apontamento
de linha na medição do L0 e renovado o hash do header; o corpo Rust é idêntico.
`reference-correction.json.gz` registra essa comparação, novo build e
revalidações de linhagem/formatação. `after.json.gz` registra o estado final.

| Verificação | Resultado |
| --- | --- |
| Sondas de tipo/valor | 24/24 passam em cada binário; antes Tekt 9/24 |
| Controles de erro/propagação selecionados | nove com exit/stdout/stderr preservados |
| Suíte core | 6.705 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.710 nomes; quatro adicionados, nenhum removido |
| Check workspace com todos os targets | passou |
| Build, formatação, diff check | passaram |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Clippy core lib/testes com -D warnings | 140 → 139 diagnósticos distintos; ainda falha |
| Linter arquitetural completo | zero erros, 381 avisos, 1.618 notas |

## Limites e continuidade

Isto não certifica paridade total de `calc.round`: a política categórica de
não finitos/overflow real diverge do oráculo em casos registrados. A notação
textual de floats grandes também diverge e não foi usada para comparar valor.
As mensagens de validação existentes continuam diferentes das do vanilla.
Não houve atualização do pin do oráculo nem do linter publicado.

A pipeline continua bloqueada pelos avisos restantes; este lote remove uma
chamada matemática proibida pelo Clippy sem introduzir supressão. Resultados
locais não substituem execução remota da matriz de release. Nenhuma release
foi publicada. Os demais itens da auditoria de produção continuam abertos.
