# Capacidade exata dos vetores de famílias — 2026-10-10

## Medição anterior à decisão

O baseline deste experimento é SHA256 `bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8`. A coleta de stacks desse mesmo binário em style-scope-transfer-2026-10-10 mostra 400 amostras de pedidos480bytes sob Vec::from_iter/StyleChain::font no caso local1600. A sonda `capacity-probe.json`, com FontFamily real e hash da biblioteca/fonte, mede120bytes por família e reproduz len1/capacity4/buffer480bytes usando filter_map/collect. É amostragem periódica inclusiva e sonda sintética de capacidade: não quantifica CPU exclusivo. `style_chain.rs:641–653` cria esse vetor com limite inferior zero. Hipótese refutável: reservar a contagem real poupa capacidade excedente; uma segunda travessia pode custar tempo.

O memo tentado imediatamente antes (`style-font-resolution-reuse-2026-10-10.md`) foi rejeitado e source/L0/binário restaurados. Esta alternativa foi medida a partir do baseline restaurado.

## Alteração e classificação

L0 primeiro, fluxo interno ADR-0127. No resolver custom text.font/Array, contar Value::Str, reservar exatamente esse número e preencher em ordem com FontFamily::new. Nenhuma memória adicional nos nós, memo ou normalização antecipada. Typed font continua prioritário; primeira ocorrência custom, tipos ignorados, arrays mixed/empty e interrupção de herança por None permanecem. Dict custom legado/regex não recebe correção semântica neste passo. Capacidade é mecânica; nomes/famílias, ordem, herança, texto e markup são observáveis da linguagem. Não inferir intenção do vanilla da sua implementação.

## Gate e medição

O gate externo foi RED no binário anterior e GREEN no candidato: três execuções por caso, **bytes solicitados menores em todos os seis controles e contagem de alocações sem crescimento**. Duas verificações semânticas foram escritas no módulo antes da implementação e passaram; 28 testes do módulo e 6.682 testes L1 passaram,5 ignorados. O gate mede bytes acumulados glibc, não memória retida/RSS.

Tempo/RSS: três rodadas de quatro pares por caso, warm-up excluído, ordem embaralhada, mesma CPU fixa, Rayon1/jobs1 e stack64MiB. Before/after têm argv com mesmo basename e comprimento. Builds, suites, gate com interposer e qualificações terminaram antes do benchmark sem interposer.

| Caso | Pedidos glibc no candidato | Redução dos bytes acumulados | Tempo pareado | RSS pareado |
|---|---:|---:|---:|---:|
| local-set-400 | 154,052 (igual) | −288,360 (0.275 MiB) | -0.30% | +0.00% |
| local-set-1600 | 328,505 (igual) | −1,152,360 (1.099 MiB) | +0.64% | +0.17% |
| no-fill-1600 | 253,234 (igual) | −360 (0.000 MiB) | +1.13% | +0.03% |
| inline-set-1600 | 459,588 (igual) | −1,152,360 (1.099 MiB) | -0.80% | +0.34% |
| variants-1600 | 1,121,236 (igual) | −1,728,360 (1.648 MiB) | -0.57% | -0.06% |
| families-1600 | 391,060 (igual) | −360 (0.000 MiB) | +1.00% | -0.07% |

Rodadas de tempo iniciais:

- `local-set-400`: -4.85%, +0.01%, +0.77%
- `local-set-1600`: -2.19%, +1.53%, +0.49%
- `no-fill-1600`: +1.55%, +0.46%, +0.55%
- `inline-set-1600`: -0.71%, -0.93%, -0.80%
- `variants-1600`: -0.69%, +1.85%, -1.13%
- `families-1600`: +1.22%, +0.54%, +1.64%

A repetição resolve o risco concreto dos controles no-fill/families positivos nas três rodadas iniciais. Mesmas condições, agora3rodadas×8pares por caso:

| Caso | Tempo pareado | RSS pareado | Rodadas de tempo |
|---|---:|---:|---|
| local-set-1600 | -0.73% | -0.53% | +1.06%, -2.12%, -0.36% |
| no-fill-1600 | -0.00% | +0.11% | -1.54%, +0.23%, +0.07% |
| inline-set-1600 | -0.91% | -0.23% | -2.70%, +0.39%, -0.54% |
| families-1600 | -0.14% | +0.42% | -0.27%, -0.36%, +0.22% |

Controles curtos e de uso não consumido,3rodadas×4pares:

- `plain`: tempo -0.25%; RSS -0.44%.
- `unused-font`: tempo -0.02%; RSS -0.33%.
- `unused-style`: tempo -1.10%; RSS -0.65%.
- `default-scopes`: tempo +0.48%; RSS -0.38%.

## Decisão final

Manter a reserva exata pelo desperdício de capacidade removido, com observáveis preservados e sem regressão temporal persistente na repetição. A melhoria certificada é dos bytes acumulados dos vetores: não declarar aceleração geral, RAM residente menor ou maior homogeneidade sem prova adicional. O memo anterior continua descartado.

## Revalidação e proveniência

25 tabelas+8 escopos preservaram texto/palavras/páginas/dimensões/bbox/raster36dpi entre before/after; seis query(table).len() produziram1 nos três motores. Preservação desse corpus não certifica paridade total nem fecha limitações anteriores. Build dev/release e lint saíram0 no candidato; avisos V5 de outros owners permanecem. Nenhum V13/V15/V26 ou drift do owner alterado.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, lista exata de arquivos/diffstat e inventário produtivo nos receipts. Somente style_chain.rs mudou no candidato.

- Baseline `bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8`; candidato `90ab169f7d583f948613f6b1f6694d9b144f7402f44406b91fdd4687981fb335`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, receita/tree P1489. Variants não tem trabalho linguístico equivalente certificado.
- UTC do primeiro benchmark `2026-10-10T04:45:13.209192+00:00` → `2026-10-10T04:46:15.683723+00:00`; repetição `2026-10-10T04:46:50.745003+00:00` → `2026-10-10T04:47:25.178526+00:00`; controles `2026-10-10T04:46:15.746197+00:00` → `2026-10-10T04:46:31.125764+00:00`.
- Arquivos, fontes, argv, ambiente, CPU e hashes de binários/scripts/fixtures: before.json.gz, allocation-equal-red/green.json.gz, measurements-equal-argv.json.gz, measurements-repeat.json.gz e measurements-cost-controls.json.gz.
- Fonte/L0 anteriores, diff e seals: before.json.gz, owner-diff.patch e seal.json. Logs completos do candidato: candidate-validation.json.gz. Estado final: final-validation.json.gz.

## Próxima investigação

Perfilar CPU do pipeline atual para separar eval, captura lexical/Styles e medições de layout. Os dois experimentos desta rodada mostram que menos bytes/pedidos não bastam para acelerar os casos locais. Não escolher cache ou nova representação de fontes antes de demonstrar a fatia efetiva de CPU e o custo residente correspondente.
