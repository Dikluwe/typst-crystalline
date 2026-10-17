# Relatório — gate HTML e identidade dos módulos

Etapa concluída. `std.html` exige agora a feature `html`, inclusive em aliases,
closures e módulos importados. Módulos comuns chamados `global` ou `format`
conservam seus próprios campos sem gates indevidos. O objetivo global segue ativo.

## Proveniência e limites

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Baseline `2026-10-17T16:24:22.231135+00:00`,
fechamento `2026-10-17T16:54:14.523992+00:00`. before.json.gz, red-state.json.gz e final-state.json.gz
registram status, diff HEAD --stat, fontes/L0 e hashes exatos. Baseline conferido
contra os 2123 hashes da etapa anterior. As medições conservam UTC, argv,
fixtures, saídas e SHA-256 dos binários. Vanilla: pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binário
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Nenhum commit, push, merge ou release.

## Correção

Dez pares consumer/L0 e um Núcleo Tekt compartilhado e pinado. A especificação
foi atualizada antes dos testes/código, em fluxo contínuo ADR-0127. `ModuleInner`
recebe identidade privada Ordinary/Std/Format. Somente as factories internas de
composição concedem Std/Format; `Module::new` sempre cria módulo comum. As três
construções reais de std e a factory de format usam essas factories. A identidade
acompanha o Arc, sem depender do nome local ou de bindings semelhantes.

O gate HTML reconhece std e format; asset/bundle conservam suas features.
O helper único é compartilhado entre acesso a campo e chamadas, antes dos
argumentos. Set preserva erros de resolução de pure_path e identifica o alvo
HTML por FuncRepr::Format(Html, _) em vez da grafia do alvo, conservando o
validador pretty existente para aliases. Condition false continua atalhando. Não
há mudança de campos, métodos ou assinaturas Rust públicos, Eq de Module,
Debug, content, metadata, setters, fases ou defaults. public-contract-before/after
confere as declarações; integrity-result.json confere que fontes fora do escopo
permanecem iguais. Tamanho observado (bytes): Module 8→8,
ModuleInner privado 448→456.
Não se alega ganho de desempenho ou memória de documentos.

## Validação

- RED funcional: três falhas antes do código e dois controles aprovados; GREEN:
  cinco testes novos. Gates nos oito perfis, aliases/closures com shadowing,
  cópia de bindings nativos em módulos comuns, igualdade/debug/estado vazio e
  precedência de diagnóstico antes dos argumentos de chamada/set nos três targets.
- 304 sondas bilaterais: 28 resultados corrigidos;
  categorias em probe-verification.json. Rejeições com feature desligada são
  registradas separadamente e não recebem crédito de paridade de feature ativa.
  Mensagem, hints e âncoras coincidem com o pin; somente o aviso já existente de
  arquivo vazio do Tekt é descontado por literal exato, preservado na coleta.
- 28 sondas adicionais: 26 casos válidos de diagnóstico, hints, âncoras e
  efeitos de chamada/set, aliases HTML ativos, alvo ausente e condição falsa;
  dois controles de rejeição sintática, sem crédito de execução de set.
  Houve 16 resultados causais corrigidos, além dos 28 da matriz de identidade.
- 460 rejeições corretas em 115 caminhos std.html nos quatro perfis
  com Html desligada; 460 controles desses caminhos nos quatro perfis
  ativos conservaram type/where/kind. Outputs vanilla reutilizados somente após
  conferir binário pinado, expressão, flags e arquivo de entrada vazio.
- Suíte workspace: 8905 aprovados, 0 falhas, 7 ignorados.
  Build, Clippy all-targets estrito, Rustdoc estrito, fmt, V1/V5/V15/V26,
  reverse lineage e git diff --check passaram.
- Diagnóstico global preservado: {'V16:error': 117, 'V19:note': 1084, 'V20:note': 873}. Nenhuma supressão ou configuração
  de lint alterada; diferencial exato após mapear deslocamentos de linhas.
- Recursos sequenciais: MemoryMax 12 GiB, High 10 GiB, swap zero e Cargo jobs=1.
  Pico de cgroup 10.002373 GiB (10739965952 bytes); eventos {'low': 0, 'high': 274, 'max': 0, 'oom': 0, 'oom_kill': 0, 'oom_group_kill': 0}.

## Pendências e tentativas

A igualdade entre módulos std de arquivos diferentes permanece divergente
(Tekt false / pin true), medida em oito controles; não foi alterada por esta
correção de gate. Permanecem os 117 V16 anteriores e as frentes RG1, FG1, RM1,
SY1 e demais contratos específicos já inventariados. RG1 continua dependente de
aprovação; candidatos futuros exigem rebase sobre o estado vigente.

A primeira sonda usava incorretamente let m = import e está arquivada, sem crédito;
as sondas válidas usam import as. Uma compilação inicial dos testes encontrou
import Rust incorreto de SysInputs; foi corrigido antes do RED funcional, com
saída e recursos arquivados. Essas tentativas não foram tratadas como prova de
falha do produto. O RED funcional foi confirmado antes da implementação. O primeiro GREEN revelou a execução prematura de
argumentos em chamadas; as sondas causais também mediram erros apagados em
set. A continuação foi prescrita nos L0 antes de corrigir estes caminhos, e
a versão candidata anterior permanece arquivada em candidate-1.
