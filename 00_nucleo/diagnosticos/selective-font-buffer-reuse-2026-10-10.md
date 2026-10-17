# Leitura direta e reserva da projeção de fontes — dois candidatos rejeitados

## Evidência antes da escolha

O perfil do baseline `90ab169f7d583f948613f6b1f6694d9b144f7402f44406b91fdd4687981fb335` (`style-cpu-profile-2026-10-10.md`) classifica54–73% das amostras de CPU de processos novos como catálogo de fontes. Essa atividade ocorre dentro de eval; não é custo exclusivo do intérprete ou de Reader. A fonte seletiva anterior (`selective.rs:78–93,178–187`) cria um Vec para cada payload e o copia para out, que cresce sucessivamente. Buffers/cópias são mecânica; metadata, cobertura, seleção/fallback e render são observáveis da linguagem. Não inferir intenção upstream.

## Candidato1: leitura direta

L0 atualizado antes do código, fluxo interno ADR-0127. Reader privado ganhou append limitado na cauda de out, sem publicação parcial na falha. Get continuou para cabeçalhos/diretório/prefixo post; offsets, limites8MiB/16MiB/4096records, requested e ordem de leitura, identidade de arquivo e fallback preservados. Nenhuma API pública, flag, fase ou cache novo.

O teste de destino do último payload foi RED→GREEN. Falhas de payload/seek/read parcial e controles de post/TTC/budget/metadata passaram. 1.281 testes infra passaram,2 ignorados. Redução fixa de7.784chamadas glibc e50.785.608bytes solicitados acumulados em cada um dos seis controles. Essa redução não mede RSS.

A repetição3rodadas×8pares confirmou inline−2,85% de tempo, mas plain+3,79% de RSS e famílias+1,94%. Portanto, o ganho de latência não legitima esse candidato para a meta de memória do projeto. Candidato SHA256 f1d42a70f141212953498d081a20d66eb1a7aca34e46ba879393f655505d0a72; provas em direct-only/, fonte arquivada como selective.rs.txt e cópia de binário de reprodução.

## Candidato2: reserva inferior conhecida

Hipótese refutável: diminuir as realocações de out poderia reduzir a regressão de RSS. A causa por fragmentação/política do alocador não foi demonstrada. L0 atualizado antes do refinamento. O diretório já lido fornece soma de head/hhea/maxp/name/OS2/cmap e min(len,32) por post, incluindo duplicatas. Soma/reserva limitada e checked antes de payloads; postv2 pode crescer no ponto original, sem prefixo antecipado ou I/O adicional. Um teste postv3 de destino de todas as tabelas foi RED→GREEN.

O segundo candidato reduziu13.633chamadas por controle, com menos bytes solicitados acumulados. Rodadas3×4pares, warm-up excluído, ordem embaralhada, CPU fixa, Rayon1, stack64MiB, before/after com mesmo basename e comprimento de argv. Tempo/RSS sem interposer; builds/probes/qualificações terminaram antes.

| Caso | Alocações | Diferença dos bytes acumulados | Tempo pareado | RSS pareado |
|---|---:|---:|---:|---:|
| local-set-400 | 154,052 → 140,419 | -114.73 MiB | -1.28% | +1.43% |
| local-set-1600 | 328,505 → 314,872 | -114.73 MiB | -1.10% | +0.76% |
| no-fill-1600 | 253,234 → 239,601 | -114.73 MiB | -2.82% | +0.09% |
| inline-set-1600 | 459,588 → 445,955 | -114.73 MiB | -0.69% | -0.10% |
| variants-1600 | 1,121,236 → 1,107,603 | -114.73 MiB | +0.41% | +0.03% |
| families-1600 | 391,060 → 377,427 | -114.73 MiB | -1.09% | +1.70% |

Rodadas de tempo:

- `local-set-400`: -1.09%, -0.81%, -2.10%
- `local-set-1600`: -1.60%, +0.92%, +0.31%
- `no-fill-1600`: -3.39%, -2.32%, -2.10%
- `inline-set-1600`: -0.31%, -1.01%, -0.43%
- `variants-1600`: -0.31%, +0.58%, +0.45%
- `families-1600`: -1.20%, -0.57%, -1.21%

Controles curtos e não consumidos,3rodadas×4pares:

- `plain`: tempo +0.76%, RSS +4.36%.
- `unused-font`: tempo -1.00%, RSS +3.55%.
- `unused-style`: tempo -2.16%, RSS +3.45%.
- `default-scopes`: tempo -2.75%, RSS -0.41%.

**Decisão: rejeitar também o refinamento.** Plain manteve+4,36% de RSS, unused-font+3,55% e unused-style+3,45%. A regressão de memória vista no primeiro candidato permaneceu após reservar o buffer. Menos pedidos/bytes acumulados não significou menor pico residente. Não atribuir uma causa exata sem medição específica. Source e L0 foram restaurados exatamente ao baseline; inventário produtivo também. O estado final e o rebuild estão em restoration.json/final-validation.json.gz. As melhorias anteriores permanecem no baseline, incluindo capacidade exata da FontList.

## Revalidação e limites

- Candidato2:14testes seletivos passaram;1.282testes infra,2ignorados. L1 não mudou e seu último resultado validado permanece6.682pass/5ignored da etapa anterior; não alegar nova execução de core aqui.
- Corpus:1140faces sistema/embutidas/fixtures;1.139aceitas e1declínio conservado. Projeção byte-a-byte e orçamento requested iguais ao baseline, metadata/cobertura iguais e comparação adicional com parser dos bytes originais. Script conserva fontes antes/depois normalizadas por caminho real, bibliotecas, compiler/receita e fonte dos wrappers/copias privadas; não é benchmark. A primeira revalidação da guarda de hashes falhou por chaves relativas de fixtures versus absolutas do inventário, corrigidas; não era diferença nos dados de fonte.
- 25tabelas+8escopos: texto/palavras/páginas/dimensões/bbox/raster36dpi preservados. Seis query(table).len() retornam1 nos três motores. Essas provas preservam o corpus, não certificam paridade total.
- Build dev/release e lint saíram0 em ambos; avisos preexistentes em outros owners. Sem V15/V26/drift do owner produtivo após resselo. A cópia arquivada .rs foi renomeada .rs.txt para não ser tratada como consumer ativo pelo linter.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Lista exata de arquivos/diffstat e inventários em before.json.gz, medições e candidate-validation.json.gz. Somente selective.rs mudou no candidato; restauração verifica todos os consumers.

- Baseline `90ab169f7d583f948613f6b1f6694d9b144f7402f44406b91fdd4687981fb335`; candidato2 rejeitado `8d36cdf5a9cc86e06be4a9e671c4fee5636fc8ebd4e616c5f3035c80a59f2ed4`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; receita/tree P1489. Variants não tem trabalho linguístico equivalente com vanilla certificado.
- Benchmark candidato2 UTC `2026-10-10T05:10:20.221184+00:00` → `2026-10-10T05:11:20.464122+00:00`; controles `2026-10-10T05:11:20.524074+00:00` → `2026-10-10T05:11:35.055957+00:00`. Repetição candidato1 `2026-10-10T05:03:59.095713+00:00` → `2026-10-10T05:05:09.231003+00:00`.
- Arquivos de medições guardam argv, fontes, fixtures, ambiente, CPU, hashes, amostras e tempoUTC. Logs/código/diff: candidate-validation.json.gz, owner-diff.patch, seal.json, corpus-validation.json.gz e direct-only/. Fonte anterior e owners exatos em before.json.gz.

## Próximo alvo concreto

O post é lido duas vezes: prefixo para determinar take e payload novamente do mesmo offset. A fonte comprova redundância de Read/Seek; o efeito temporal ainda não foi isolado. Próximo experimento: reaproveitar o prefixo já adquirido e ler somente o restante necessário, com L0 primeiro, orçamento registrado de novo, fallback/identidade preservados e novos controles de erro/byte equivalence. Isso deve medir syscalls reais e latência/RSS, sem assumir que a porcentagem de catálogo se converterá em ganho. Não foi implementado nem ratificado nesta etapa.
