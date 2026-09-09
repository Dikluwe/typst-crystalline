# P1321 — GO independente anterior ao patch

GO para implementação estritamente no recorte L0 P1321. Não é aprovação final,
selo ou equivalência geral. Revisor não escreveu produto, testes ou freeze.

## Causalidade e entradas conferidas

- Freeze A/B limpo `p1321-ab2-freeze.json`, SHA-256
  `b90c53cc2d982160dce2faca2f4afb6910d07b06e321b8bf8090bc51ead0d6ca`.
- L0 sem linha canônica Hash do Código:
  `2f6dcab26aefce4a7db207bd347b46998dd845e7205954846be4512b5c2f2cc3`.
  Conteúdo normativo apto. A sugestão Markdown não foi aplicada após freeze;
  texto literal continua inequívoco e nenhum ajuste cosmético justifica mutação.
- RED `p1321-unit-red.json`, SHA-256
  `d750796583662024a5aac54f07bf601c87c9ea50dec145ae0b0282d7c758e8d8`.
  Comando `cargo test -p typst-core --release compiler::stdlib::loading::tests --lib`,
  target `/tmp/p1321-target.669PuL`, HEAD
  `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado com
  fontes integrais/diff/stat no recibo. Executado entre
  2026-09-08T20:26:09.792866+00:00 e 2026-09-08T20:29:09.743543+00:00.
- Módulo inteiro de testes desde `#[cfg(test)]` no RED SHA-256
  `42219b7843fbcfb14b5241cd0d1b7b88be050d42a7b084c5820ed49cf59542fd`.

Conferi todos os hashes de entradas e binários pinados pelo freeze. Comparei
diretamente o prefixo produtivo da fonte RED ao texto congelado P1319 no
`p1321-baseline.json`: igualdade literal; nenhuma implementação P1321 existia.
Fonte antes/depois RED e fonte corrente também coincidem. O teste compilou;
exit 101 corresponde a 66 testes passando, seis falhando, nenhum ignorado.

As falhas são substantivas: missing antigo, unknown vencendo cast e excesso,
parser vencendo excesso e `World::read_path` proibido no controle de extra Path.
Não há erro de compilação mascarado como RED. Loops podem interromper na
primeira combinação falha; o GREEN final deve completar todas as combinações.

## Qualidade dos controles

Li o delta integral relativo à fonte P1319. Os três testes novos distinguem
span completo/value_span/args.span; named source primeira ocorrência/detached;
ordens de remanescente e fallback sintético; cast→delimiter→row-type sobre
unknown e excesso. O teste P1319 manteve suas asserções de parser/root/vpath/
leitura única sem excesso, e acrescentou a rejeição de excesso antes de I/O.
P1316 mudou só o caso de parsing com excesso; os demais controles de origem
continuam. P1313 atualizou somente missing e precedência unknown/cast.

O A/B contém 110 casos em quatro perfis, 440 expectativas: 85 casos por
referência vanilla integral, 23 por preservação baseline, dois Symbol por
expectativa normativa declarada. As 880 execuções de baseline/vanilla estão
registradas, com zero Unknown nas expectativas. Li o runner: a comparação
conserva exit/stdout/stderr integrais; Symbol tem montagem explícita de seu
diagnóstico separado, sem retirar spans/sufixos para alegar paridade.
Casos incluem alias, With, Args, sink, map, spreads, opções sobrescritas,
colisões de I/O/parser e controles de outros loaders.

## Limites de segregação

Mantém-se A/B executado sem atestação técnica de isolamento. A etapa
exploratória descartada produziu `p1321-ab-runner.cjs`, `p1321-ab-cases.json`
e temporário VBRiJC, sem execução de binário ou freeze segundo o coordenador;
nenhum desses artefatos compõe aceitação. O testador limpo informou cache
próprio de py_compile criado fora da allowlist e removido exclusivamente por
ele; não houve leitura de código candidato. É incidente de capacidade
declarado, não justificativa para atestar isolamento. A revisão aceita
evidência funcional limitada e entradas congeladas, não selo formal.

Não houve revisão focal sem ganho repetida. Próximo gate: implementação,
GREEN e A/B normal/repeat/reverse, preservação desses testes/norma, build/lint
e hashes A efetivo com Núcleo + B reverso recalculados independentemente.
