# D08B1 — projeção de célula existente; D08B2 preparado

D08B1 materializado em fluxo contínuo ADR-0127. D08B permanece aberto.

## Medição anterior à decisão e proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
`table-cell-realizacao-2026-10-07/before.json.gz` registra UTC, diff HEAD --stat,
status, hashes de fontes/binários, comandos/cwd e resultados brutos.
Baseline UTC 2026-10-07T03:53:25.954869+00:00 a 03:53:38.227059+00:00.
Oráculo pinado `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Tekt anterior SHA-256 `273cedc0f41c30e6927bbe60eaacdead106db1d00ed071e88807917c0cb1d88e`.

Upstream model/table.rs:734–778 declara body/propriedades; Tekt
entities/elements/table_cell.rs:62–109 tinha getter default;
content/identity.rs:107 usava table.cell; content_methods.rs:226–254 não
projetava função efetiva; rules/show.rs:300–320 omitia native_table_cell.
Getters, fields, func-eq e show explícito refutaram paridade. São semântica e
morfologia da linguagem, sem exigência de estrutura Rust ou bytes PDF.

## Decisão e implementação

Getter da entidade expõe body e valores já conservados. Content delega;
content_methods enumera os dez campos e distingue Set/Unset/Undeclared;
acesso direto mantém span e diagnóstico de campo declarado ainda desconhecido.
Nome público é cell; Debug mecânico conserva table.cell. Func projeta a nativa
real; show direto reconhece sua identidade estática. Não há NodeKind cell,
registry, alteração de contrato público nem mudança de fase nesta correção.

Seis owners 1:1 foram atualizados antes de código, com núcleo compartilhado
`00_nucleo/prompts/_nuclei/table/cell-projection.toml` e resselo restrito.
`red.log` registra falha real do novo teste de body; `core.log` registra GREEN.
Os arquivos fontes/L0 anteriores permanecem no snapshot baseline-state;
tested-final-code/final-code e seals.json registram o estado final exato.
O recibo final-results.json confirma zero alteração produtiva/L0 fora do escopo.

## Resultado, sem fechamento excessivo

Tekt final SHA-256 `53cf87137baa268e30effaef978455611b3e2dbdfba90d43d1f446e5eef0e456`.
`after.json.gz` e `final-results.json` contêm UTC/HEAD/diffstat/fontes e coletas
integrais, incluindo os negativos. Na coleção de 35 controles, coincidências
integrais passaram de 1 para 11. Body, fields cru, x omitido com diagnóstico,
x/y/spans explícitos, identidade/func-eq, ordem de keys, show filtrado por x e
show fora da tabela coincidem. Show identity coincide em texto extraído, o que
não comprova observação dos campos resolvidos.

Show direto e selector explícito agora executam e acessam body, mas o texto
PDF conserva diferença de espaçamento entre células. Não certificar render
por apenas normalizar whitespace. Raw-options conserva desvio de repr do
stroke (2pt com paint default explícito); correção fica nomeada no D08B2.
Auto/none lexical, células implícitas, padding, estilos/posições resolvidos,
reuso, nested e header/footer permanecem negativos explícitos. Não fechar D08B.

## Validação do estado final

As contagens abaixo provêm dos logs e snapshots do diretório desta execução;
source hashes, HEAD, diffstat e UTC por coleta estão em final-results.json.

- Core: 6.583 passaram, 5 ignorados; infra: 1.194 passaram.
- Shell query: 5 passaram; integração query: 10 passaram.
- Cargo build debug e release concluídos; git diff --check sem problemas.
- Crystalline-lint: zero erros, 532 warnings e 1.601 notas. Warnings legados
  permanecem; não usar resselo global para escondê-los.
- D08A: 21/21; seus dois controles D08B históricos continuam fora desse escopo.
- D05: 29/29; D07: 29/29 + 14/14; query bilateral: 36/36.
- D06: 38/41; root-render, raw-eval e raw-mat-eval continuam resíduos conhecidos.

## Próximo contrato concreto

Os 18 refutadores auto/none em presence-refuters.json fundamentam D08B2.
A proposta completa está em
`00_nucleo/materialization/table-cell-presenca-2026-10-07.md`:
novo `TableCellPresence` público e campo presence em TableCellElem, preservando
os tipos dos valores existentes; constructors, transformação, observação e
serialização conservam os estados nominais. Há 15 candidatos L0 completos
com hashes atuais/candidatos e núcleo próprio no manifesto SHA-256
`35f73c60301e1e4b27ae8b3ed2928b6d421dbcd0f32502131c3e2458260301e6`.
São candidatos não instalados nos L0s canônicos; nenhum código D08B2 foi escrito.
ADR-0127 exige confirmação por adicionar campo/tipos públicos. D08B3 de
realização por posição/estilo/padding é proposta posterior separada; a aprovação
de presença lexical não ratifica fase, paints/inset ricos nem query locatable.
