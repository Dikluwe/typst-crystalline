# Encerramento da sessão R3-T1 para organização do projeto

Encerramento solicitado pelo usuário; correções pausadas após validação final.
Este é um marco parcial validado, não a conclusão da paridade nem do objetivo
“Faça todas as correções”. Não houve commit, push, release, limpeza ou remoção de arquivos.

## Proveniência e escopo

Branch `Tekt`, HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, working tree não commitado.
Encerramento UTC `2026-10-18T14:43:17.759514+00:00`. O recibo `estado-encerramento.json.gz` conserva
status, arquivos não rastreados, diffstat exato e hashes do código no encerramento.
As medições possuem seus próprios freezes, horários, comandos e executáveis:
`before-sources.json.gz`, `candidate9-sources.json.gz`, `candidate10-sources.json.gz`
e os recibos `*-resources.json`. Os resultados da revisão 9 não foram
reclassificados como medições da revisão 10.

Aprovação: `00_nucleo/materialization/textual-realization-2026-10-18/approval-receipt.json`,
manifesto aprovado `47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302`.
A etapa altera 33 arquivos em relação ao seu freeze inicial:
16 Rust, 16 prompts L0 e 1 núcleo.
Lista e hashes em `escopo-etapa.json`; não confundir com todo o diff acumulado do projeto.

Oracle medido: vanilla `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
binário SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Linter pinado `12797073972e375d9ab078cf6750c8b30b71e2b9`.

## O que foi implementado

- Realização do pai antes dos filhos que sua regra show pode ocultar; coordenação
  causal privada em `compiler/realize/recipes.rs`, com owner L0 próprio.
- Distinção entre regras herdadas no ponto de inserção e regras locais do conteúdo.
- Precedência conjunta de literal/regex, seleção por posição e ordem lexical,
  produtos protegidos contra reaplicação e fronteiras de agrupamento preservadas.
- Show-set textual, restauração de ambiente nos retornos/erros e preservação de
  spans e identidade de aliases; integração com Sequence, Table e matemática.
- Correções de integração para fluxo de parágrafo, hifenização e contextos já
  cobertos pela suíte. Não equivalem a paridade universal de Context.
- Remoção dos dois novos V17 por desdobramento de guards, sem silenciamento.
  A revisão 10 só altera dois owners Rust e seus L0s em relação à revisão 9.
  Não foram introduzidas novas assinaturas públicas.

L0s foram atualizados antes do código e resselados. A auditoria final conserva
ownership e núcleos válidos; o resselo não autoriza ignorar V16.

## Validação e limites

| Evidência | Resultado | Estado medido |
|---|---|---|
| Suíte completa, incluindo doctests | 9111 aprovados, 0 falhas, 7 ignorados | revisão 10, `suite-summary-candidate10.json` |
| Build e Clippy workspace/all-targets estrito | passaram | revisão 10, logs e `closure-candidate10-resources.json` |
| Rustdoc estrito, fmt, diff-check, linhagem direta/reversa | passaram | revisão 10, logs `*-candidate10.log` |
| V17 e V1/V5/V15/V26 | passaram | revisões 10, preflight e fechamento |
| PDF e HTML do corpus textual | 56/56 em cada target | revisão 9, `exports-check-candidate9.json` |
| Cores de show-set em SVG | 8/8; mais 6/6 de ordem entre funções e estilos | revisão 9, exports e `style-order-candidate9.json.gz` |
| Regressões de layout anteriores | 297 casos / 327 artefatos, 326 preservados e 1 correção comprovada | revisão 9, `mixed-regressions-candidate9.json.gz` |

Os 56 casos comparam aceitação, texto e erro principal; não provam geometria
PDF completa. A alteração de layout foi o alias de bibliografia: o recuo após
show passou de 26 para 44 pt, coincidindo com o vanilla. A prova específica está
em `alias-proof-candidate9.json.gz`; os demais PDFs foram comparados por
páginas/texto/geometria e SVGs por bytes. Estes corpora externos não foram
reexecutados na revisão 10; sua alteração de guards foi revista e a suíte
completa foi repetida. Os logs RED, erros de fixtures e candidatos intermediários
permanecem preservados; não são evidência GREEN.

Tekt global continua vermelho pelos **105 V16 herdados**. Resultado final:
105 V16:error, 1144 V19:note e 879 V20:note; nenhum novo erro/warning em relação
ao início desta etapa. `audit-candidate10.json` e `architecture-candidate10.log`
registram o diferencial. V19/V20 são observações, não bugs automaticamente.

## RAM

Fechamento executado sequencialmente com um job, MemoryMax=12 GiB,
MemoryHigh=8 GiB e swap zero. Pico do fechamento: **5.036 GiB**;
pico da validação ampla da revisão 9: **4.619 GiB**.
Eventos completos nos recibos, sem OOM/OOM-kill nesses dois gates.

`ram-probe.json.gz` conserva a medição exploratória do produto na revisão 9.
Ela não demonstrou redução de RAM: o consumo ficou próximo do Tekt anterior.
Tekt foi medido em debug e vanilla em release, com duas repetições; essa amostra
não permite declarar ganho de produção ou comparação justa de desempenho.

## Pendências confirmadas de Context

As sondas adicionais da revisão 9 encontraram quatro divergências herdadas.
O controle de uma equação passa; os demais continuam abertos. Antes e depois
reproduzem a divergência; não são regressões introduzidas neste lote.
Evidência com fontes, comandos, horários e binários: `context-frontiers-candidate9.json.gz`.
Não houve nova medição externa destas sondas na revisão 10.

| Sonda | Tekt anterior | Tekt revisão 9 | Vanilla |
|---|---|---|---|
| equation-one | `EQ Q1` | `EQ Q1` | `EQ Q1` |
| equation-two | `EQ EQ Q1` | `EQ EQ Q1` | `EQ EQ Q2` |
| equation-heading-prefix | `H EQ Q0` | `H EQ Q0` | `H EQ Q1` |
| text-context-global | `A` | `A` | `R` |
| text-context-local | `A` | `A` | `L` |

Contagem/identidade de equações em contextos e regras textuais globais/locais
em Context são pendências de linguagem, não divergências mecânicas Tekt aceitas.
Retornos de callbacks fora de Content/String (R3-T2) também continuam abertos.
O corpus aprovado não cobre toda a linguagem. O restante do inventário e as
pendências de layout anteriores continuam válidos; ver `proximos.md`.

## Entrega para a limpeza

Ponto de entrada: `proximos.md`. Preservar código, L0s, aprovações, fontes dos
casos e evidências referenciadas. `outputs/` e `bin/` têm arquivos ignorados pelo
Git: não presumir que um commit os preserva. Classificar os artefatos antes de
remover; esta sessão não os apagou nem iniciou novos reparos.
