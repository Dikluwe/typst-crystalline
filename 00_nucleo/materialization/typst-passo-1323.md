# Passo 1323 — completar o warning experimental HTML

## Resultado pretendido

Entregar a headline e os três hints do warning experimental HTML, na ordem
e com a terminação medidas no vanilla ratificado `a51e02804`. Não alterar
conteúdo HTML, feature gate, formatos paginados ou o hint dinâmico de query.

## Medição e decisão

P1322 selecionou `html-experimental-warning-hints`, prioridade 2: o L0
`00_nucleo/prompts/wiring.md:171–173` já promete o warning medido no vanilla,
mas `04_wiring/src/main.rs:388–392` imprime só a headline. A fonte vanilla
`lab/typst-original/crates/typst/src/lib.rs:249–256` constrói três hints.
Seleção sucessora: `00_nucleo/diagnosticos/p1322-classification-selection-r2.json`,
SHA-256 `26f24e212bffa82ca9f3e48f8e46ce37c414cb514c96ae16cacaaea1050fc345`.

A medição fresca, anterior ao L0/código deste passo, está em
`00_nucleo/diagnosticos/p1323-baseline.json`, SHA-256
`8abba2b5f0e3280b4632e4d3bec1db6c2c7a8ee55ab3e9bc9d0d44f197eaa897`:
argv, fontes, UTC, saídas, HEAD e diff integral. O HEAD é
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, com as quatro alterações
anteriores loading.md/rs e call_dispatch.md/rs preservadas. Não commitar,
descartar ou ressellar essas mudanças por conveniência.

Mensagem/hints são observáveis diagnósticos (ADR-0107/0108). A escolha de
armazenar um literal não é critério de paridade. A intenção vem da promessa
específica L0, não de inferir que todo comportamento observado seja desejado.
Classificação: correção interna de paridade ADR-0127, fluxo contínuo com
L0-first + hashes + RED→GREEN + revisão.

## Escopo

Owner produtivo único: `04_wiring/src/main.rs`, legitimado exclusivamente
por `00_nucleo/prompts/wiring.md`. Núcleos e outros owners não mudam.
Uma constante privada pode nomear o literal já emitido, permitindo teste
no próprio módulo; não criar formatter, algoritmo, tipo ou API pública.
Se necessária preparação mecânica para o teste, conservar exatamente a
headline anterior e confirmar equivalência no baseline antes da correção.

Proibido: mudanças de target/default/features, serialização, pipeline,
query/help, lógica L1–L3, formatter genérico, atualizações de vanilla,
mutação de evidências antigas, stage/commit/push e limpeza de temporários.
Necessidade de outro owner/API/fase refuta o escopo e exige reabrir o gate.

## Execução e aceitação

1. Congelar baseline e atualizar o L0 com envelope e preservações explícitos.
2. Usar A/B conforme a skill de materialização segregada: root escreve L0 e
   implementação; autor de testes recebe apenas L0/vanilla/fixtures e não lê
   patch; revisor separado julga L0, testes, execução e diff. Capacidades são
   declaradas, sem atestação técnica de isolamento ou selo de refinamento.
3. Autor A/B congela testes unitários e de CLI antes do candidato semântico.
   Teste unitário deve falhar por conteúdo ausente, não por erro de compilação.
   Teste CLI deve observar stdout/stderr completos, exit, fonte e artefato.
4. Corrigir somente a emissão fixa: três hints e terminação, mesma headline,
   canal, condição e ordem de execução. Não esconder outros warnings.
5. Reexecutar testes independentes normal/repetida/inversa: HTML ativo nos
   perfis html e combinado; negativo sem HTML/default/a11y; grafia legada e
   compile explícito; ambos modos cristalinos de serialização; controles
   PDF/SVG/PNG e query preservados frente ao baseline. Separar igualdade
   vanilla do warning de preservação do artefato e de outros diagnósticos.
   Não exigir bytes de PDF como paridade. Nenhum caso desligado ganha crédito
   de funcionalidade HTML exercitada; negativo mede a ausência do warning.
6. Opacidade/timeout/crash/saída obrigatória ausente = Unknown bloqueante.
   Testes do discriminador devem rejeitar hints removidos/reordenados,
   duplicação, canal trocado e falta de terminação em cópias dos observáveis.
   Isso não constitui mutation score produtivo nem quita dívida histórica.
7. Rodar build/test workspace release --locked, fmt --check, lint,
   V5/V15/V26 e diff --check; publicar avisos/ignorados. Verificar que apenas
   o par proprietário mudou além dos quatro arquivos anteriores intactos.
8. Relatório substantivo em `00_nucleo/diagnosticos/p1323-final-report.md`,
   com limite de cobertura e veredito separado. Não escrever P1324.

Temporários: target exclusivo `/dev/shm/p1323-target.M0xPMi`, cache copiado
sem hardlinks do target P1322; acesso escalado específico por a visão sandbox
montar RAM read-only. Não usar targets ou outputs de outros passos como destino.
Budget de calibração: até duas revisões focais sem ganho antes de rever método;
não repetir corpus inteiro enquanto a causa focal não melhorar.

O relatório final registra a conclusão; este plano não antecipa resultados.
