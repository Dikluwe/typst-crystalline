# L0 — compiler/eval/call_dispatch/arguments
Hash do Código: caaf2085

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/arguments.rs`. No HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` e árvore limpa, `01_core/src/compiler/eval/call_dispatch.rs:390-502` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contrato

Avaliar argumentos e spreads na ordem de AST e transportar os spans de chamadas nativas. Preservar assinaturas públicas, diagnósticos, prioridades de intercepção, rastros e semântica de argumentos especificados em `00_nucleo/prompts/compiler/eval/call_dispatch.md`. A extração permanece em L1 com despacho estático, sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1, os testes de call_dispatch e `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

## D08B3-A — tabela resolvida na inserção; proposta pendente ADR-0127

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07/
{before,additional,order,aliases}.json.gz registra pin 9f2b6e8715237cb086899a42873660fe744622e8,
UTC, diffstat/status, fontes/binários, argumentos e resultados. Oracle binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea; Tekt
7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2.
Vanilla lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278 resolve campos, :1500–1530 rejeita
conflitos na ordem, :1665–1703 cria vazios, :1945–1988 deriva breakable.
Tekt grid_placement.rs:91–112 prioriza explícitas e :262–270 só conhece GridCell;
realize/mod.rs:160–200 visita receitas bottom-up; table.rs:127–152 despeja
wrappers sem atributos. order mede zero efeitos de células quando a tabela é
substituída e zero efeitos de heading quando a célula o oculta; aliases mede
LOCAL a, OUTER LOCAL a e NEW a. É linguagem, sem copiar mecânica/bytes/igualdade
Rust. Hipótese refutada por campos desconhecidos, efeito oculto ou ordem errada
após a mudança; cache/paginação não são inferidos como equivalentes.

### Responsabilidade exclusiva proposta

Transportar call.span completo para native_table/native_table_cell por identidade tipada (inclusive With já suportado), conservando value_span individual para casts. Source causal dos novos campos vem de Args.span; não usar nome como discriminador.

### Gate, aceitação e divisão explícita

Proposta não vigente até confirmação humana do manifesto: há cinco campos públicos,
dois carriers públicos e uma fachada pública nova, além da fase lógica antes de Cell
show. Após ratificação, RED→GREEN, suites/build/lint/preservação restrita e coletas
D08A/B2, D05/D06/D07/query obrigatórios. Aceitação em linguagem/efeitos/geométricas
reais, nunca igualdade de bytes ou normalização de whitespace para fechar render.
Este primeiro pacote D08B3-A é incompleto no universo Cell: D08B3-B completa defaults
de Cell isolada (raw-cell-no-placement/outside-fields) e decisão separada de geração
fresca show-cell-reconstruct. Defaults/fields genéricos de Table (table-root-equality),
styles funções/paint/strokes/insets ricos, linhas/gutters ricos, spans multipágina e
conformidade PDF/UA continuam fora da certificação. Não mudar casts legados extras
do D08B2, sintaxe, flags ou política não-localizável global neste pacote.


## Guards de domínio explícitos — V17, 2026-10-16

### Medição anterior à decisão

T011: 01_core/src/compiler/eval/call_dispatch/arguments.rs:99. Fontes e L0 vigentes lidos antes da prescrição; SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, UTC, status e git diff HEAD --stat em `00_nucleo/diagnosticos/tekt-guards-2026-10-16/before.json.gz`. O inventário anterior, reconfirmado por hash, registra 28 guards em 21 owners; a regra CI pinada manda desdobrar ou mover a condição para o corpo. A medição é estrutura Rust, não prova de defeito funcional ou de melhor leitura por IA. Nenhuma intenção upstream é inferida.

### Responsabilidade exclusiva e aceitação

Conservar With recursivo e caminho p1339 prioritário; Native decide no corpo se a identidade de função pertence à lista existente, na mesma ordem de fn_addr_eq e sem nomes textuais. Somente os mesmos oito alvos transportam args.span; demais representações permanecem sem efeito.

Manutenção interna em fluxo contínuo ADR-0127: sem contrato público, default, compatibilidade ou fase novos. A responsabilidade permanece somente no consumer exclusivo deste L0. Não criar helper apenas para esconder a conjunção, não adicionar supressões nem mudar a configuração do linter. Executar controles funcionais existentes antes do código e preservar seus esperados; o RED real é o gate V17 com 28 avisos, GREEN exige zero V17. Conferir caminhos de sucesso/fallback/erro, curto-circuito e efeitos; resselo restrito por owner. Revalidar build, suíte workspace completa, Clippy all-targets com warnings e expectativas não cumpridas como erro, formato, Rustdoc com warnings como erro, linhagem V1/V5/V15/V26 estrita/reversa e SARIF sem novos avisos. Executar sob 12 GiB, swap zero e um job. Não alegar ganho de memória/desempenho ou paridade global sem medição separada.

## R2 — origem do construtor Sequence

Medição antes da decisão: r2s-scope.json.gz em
00_nucleo/diagnosticos/sequence-constructor-2026-10-17 conserva estado
não commitado em b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, UTC,
diffstat e binários no pin bd5ebec285505889e8ff1d4f68804ab147a699f3.
missing e unknown-only têm mensagem igual, mas o Tekt ancora apenas os
parênteses, enquanto o vanilla ancora f() / f(x:1) completos.
arguments.rs:84–132 só transporta call_span para a whitelist tipada
anterior, que não inclui native_sequence; content_methods.rs:355–359
usa Args.span para ausência do argumento obrigatório.

Correção interna de mapeamento: acrescentar exclusivamente a identidade
tipada de native_sequence à whitelist de transporte de call_span,
incluindo With pelo caminho recursivo vigente. Conservar os value_span
individuais e todas as prioridades e efeitos. A cláusula anterior de
oito alvos passa a incluir este novo construtor, sem mudar outros alvos
ou assinaturas. Testar os ranges completos de f(), f(x:1) e o valor
individual incorreto; RED real antes do código e GREEN bilateral.
