# Passo 1317 — mensagem clara para CSV com UTF-8 inválido

Estado: escrito, implementado e validado; revisão independente PASS no recorte. Sem commit.

O CSV com bytes inválidos ainda expõe a mensagem interna do parser, embora
a origem Bytes já esteja correta. Corrigir somente a causa textual para
`failed to parse CSV (file is not valid UTF-8)`, tanto em cabeçalho quanto
em dados, sem adicionar posição/caminho ou alterar a precedência do parser.

Medições anteriores à decisão: `00_nucleo/diagnosticos/p1317-measurement.json`
e `00_nucleo/diagnosticos/p1317-measurement-focal.json`. O passo coordena;
o L0 proprietário é `00_nucleo/prompts/compiler/stdlib/loading.md`, consumer
único `01_core/src/compiler/stdlib/loading.rs`.

1. Atualizar L0 e revisar fluxo contínuo ADR-0127.
2. Congelar A/B independente; provar RED com testes locais novos.
3. Materializar somente a formatação de ErrorKind::Utf8; provar GREEN.
4. Build, testes workspace, lint/linhagem, A/B e revisão independente.
5. Relatar efeito prático, evidência reproduzível e dívidas em diagnósticos.

Regime A/B sem atestação técnica de isolamento; não há selo geral ou mutation
score. Root escreve L0/código/testes locais; testador independente recebe L0
e binários, sem candidato; revisor julga sem editar artefatos julgados.
Uma medição/freeze, correções focais antes de repetição integral e candidato
normal/repeat/reverse. Duas revisões sem ganho na mesma causa obrigam rever
o desenho, não afrouxar gates. Unknown bloqueia.

P1315/P1316 não commitados devem permanecer preservados. Não fazer commit,
stage, push ou limpeza. Target exclusivo `/tmp/p1317-target.y5u9ah`, cache
copiado sem hardlinks; RAM livre insuficiente para cópia independente.

## Entrega

O mapper CSV agora trata ErrorKind::Utf8 com a mensagem contratada. O erro
de quantidade de campos mantém a precedência, e as origens P1316 permanecem
preservadas. RED→GREEN comprovado; build, 6.656 testes (três ignorados),
4.644 comparações A/B e lint sem erros (240 warnings) passaram.

Proveniência, recibos, parecer independente e dívidas restantes em
`00_nucleo/diagnosticos/p1317-final-report.md`. Sem alegação de paridade geral
CSV ou isolamento técnico atestado. P1315/P1316 não commitados preservados.
