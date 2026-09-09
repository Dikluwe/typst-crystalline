# Passo 1336 — diagnóstico de field ausente em instâncias Int/Str

## Problema e efeito esperado

P1335 recomendou `primitive-instance-field-diagnostic`, após comparar as causas
do produto inteiro. A medição fresca P1336 confirma: `(1).nope` publica `int`
e sublinha o acesso inteiro; vanilla publica `integer` e sublinha só `nope`.
String tem a mesma causa, com `str` versus `string`. O erro deve apontar o campo
correto e usar o nome público, sem criar campos nem alterar valores válidos.

Origem: `00_nucleo/diagnosticos/p1336-baseline.json`, SHA-256
`e81e034167a994ab3dd1c318a7dd7384cd18ef75e0d2b3c1c666eb26724e441f`;
medição `p1336-measurement.json`, SHA-256
`0865c045fde6b0d3689a77d83a9d255ea927e76185ad7cfb0dcb6dfb8ca3e375`.
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitada;
diff/stat integral e todos os arquivos anteriores protegidos no baseline.
Vanilla ratificado upstream/main a51e02804, sem resync.

## Escopo autorizado e L0 proprietário

Atualizar primeiro somente
`00_nucleo/prompts/compiler/eval/bindings/field_access.md` e materializar em
`01_core/src/compiler/eval/bindings/field_access.rs`, incluindo testes locais.
O passo coordena; não substitui o L0. A cláusula canônica vigente já exige
`integer`; o novo amendment explicita a sucessão de mensagem/origem Int/Str.
Classificação ADR-0127: correção interna de paridade, sem parada adicional,
L0-first + resselo + RED→GREEN. Novo owner/API/default/fase exige reabertura.

Preservar métodos existentes, Type::Int/Type::Str, Bool e demais valores,
Module/Dict/Content/LocatedContent/Float, nativas/closures/With, gates PDF,
warnings e ordem de avaliação. Não corrigir `.nope(panic(...))`, math,
carregadores, serialização, novos membros ou formatter. Dívidas preservadas
continuam dívidas; igualdade com baseline não vira paridade vanilla.

## Execução e segregação

Usar a skill `tekt-materializacao-segregada` em regime A/B, com oráculos/testes,
implementação, ataques e veredito separados; sem selo de refinamento e sem
atestação técnica de isolamento. Contexto novo para os papéis auxiliares.

1. Congelar baseline, intenção L0, capacidades, política de Unknown e suite
   bilateral antes do código candidato. L0 integral e ADRs precedem decisões.
2. Autor de testes deriva do L0 casos diretos, aliases, campos variados,
   Unicode/multilinha, lookup puro com span recebido, sucessos e exclusões.
   Classifica previamente toda expectativa como convergência ou preservação.
3. Integrar testes no módulo antes do patch; observar RED real. Implementador
   não lê os testes privados antes de produzir sua solução pelo L0.
4. Corrigir apenas seleção local de âncora e nome do erro Int/Str. Resselar
   somente após validar ownership/pins e conferir o delta do reparador.
5. Executar a matriz nos quatro perfis default/html/a11y/html+a11y, ordem
   normal/repetida/inversa, canais íntegros. Cada rodada tem destino exclusivo.
6. Adversário congela mutações antes do candidato e executa mutantes produtivos
   em cópias exclusivas, nunca sobre a árvore principal. Revisor julga validade
   e rejeição, sem confundir falha de compilação com mutante eliminado.
7. Build/testes workspace release --locked, fmt --check, crystalline-lint,
   V5/V15/V26 estrito e git diff --check. Revalidar os arquivos protegidos.

Budget inicial: até oito famílias adversariais focais; uma rodada completa por
ordem sobre cada binário válido; timeout 30s por processo CLI, 2700s por comando
de build/testes. Duas revisões sem ganho na mesma falha exigem revisão do método.
Unknown obrigatório, origem opaca, crash, identidade incorreta e resultado
instável bloqueiam fechamento; opacos de calibração não contam como sucesso.

## Artefatos e entrega

Somente o par L0/consumer acima, este passo e artefatos novos
`00_nucleo/diagnosticos/p1336-*` podem mudar. Preservar a árvore suja anterior,
os diagnósticos históricos e os temporários existentes. Sem stage/commit/push.
Usar target exclusivo `/tmp/p1336-target.QiOMGq`, cache copiado sem hardlinks;
`/dev/shm` foi observado montado somente leitura neste ambiente.

Relatório final em diagnósticos começa pela diferença corrigida, testes e
fronteiras ainda abertas; inclui regime, gates reais, mutantes executados,
incidentes e fechamento verificável. Não repetir o inventário global nem
transformar este recorte em alegação de paridade geral.
