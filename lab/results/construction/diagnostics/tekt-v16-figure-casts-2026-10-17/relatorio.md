# Relatório — rejeições de figura e cobertura dos casts

Etapa concluída com correções funcionais em `figure` e **três V16 corrigidos, de 120 para 117**. O V16 da inferência parcial de kind continua visível: não foi silenciado com uma lista de containers neutros. O objetivo global permanece aberto. Sem nova API pública, mudança de fase, commit/push do Typst, merge ou release.

## O que mudou

`native_figure` agora rejeita body inválido em vez de criar conteúdo vazio, caption inválida em vez de pintar o nome do tipo, e kind inválido em vez de cair em inferência. Funções arbitrárias, With e valores que só imitam o nome de um elemento são rejeitados como `expected element`; a classificação usa a tabela de identidade nativa existente. Tipos com construtor são recusados como elemento e tipos sem construtor preservam seu diagnóstico específico. Símbolos conhecidos como acento/LR produzem expected element, outros produzem symbol {valor} is not callable, usando helpers privados nos owners de acento/LR e bridges diretos no hub. None é corpo vazio e ausência de caption, mas é inválido como kind. Strings, conteúdo e nomes explícitos válidos são preservados. Ausência de body usa `missing argument: body`.

Símbolos em body/caption preservam seu grapheme efetivo como Text. Isso melhora a preservação do conteúdo; SY1 permanece aberto para SymbolElem/reflexão/morfologia mediante revisão pública. LocatedContent é aceito como conteúdo em body e caption, conservando o conteúdo e sua origem Source. Os erros seguem body→caption→kind, mesmo se a ordem dos argumentos nomeados for invertida. A âncora vem do value_span da ocorrência correspondente, com fallback explícito; não é deduzida por igualdade Value. Os argumentos não são reavaliados.

Nos dois conversores privados, os cinco RHS existentes são preservados. As outras 34 variantes Value de cada projeção são explícitas e retornam None para o caller validar. Isso elimina dois V16 e exige classificação de uma variante futura, sem confundir a cobertura estrutural com semântica correta de percentuais. Os grupos V19 têm essa responsabilidade local prescrita no L0 e permanecem como notas, sem supressão.

A projeção de legenda presente agora devolve o carrier FigureCaption existente, conservando seu corpo e deixando defaults não resolvidos. A lógica fica no elemento; Content::get_field apenas delega. Seis sondas de reflexão corrigem três casos (legenda vazia, conteúdo e constructor explícito), preservam body e registram as duas lacunas de omissão/none. Não foram adicionados campos ou métodos de trait; o override usa a assinatura já existente.

## Verificação

Houve **14 RED funcionais reais**: quatro grupos de rejeição/precedência, um caso de conteúdo localizado, dois casos simbólicos, uma projeção de legenda, cinco casos de getters/metadados/encoders/CLI e um caso de casts do constructor de caption. Os **16 testes novos** passaram: esses 14 RED e dois controles funcionais. As sondas estruturais dos dois matches aceitam os enums atuais e rejeitam variante futura com E0004; os padrões antigos aceitavam a variante. Elas usam padrões produtivos, nomes/aridades reais e payloads stub/RHS apagado, sem mutar o workspace Cargo; não são prova de execução equivalente.

**130 pares bilaterais repetidos**: 38 casos de diagnóstico foram corrigidos e coincidem com o pin em código de saída, mensagem principal, primeira âncora e hints medidos. O complemento mede outros 70 pares: 47 diagnósticos Type/Symbol coincidem com o pin, 20 controles conservam o texto simbólico e três construtores de símbolo inválido são preservados. O tipo de nó Text versus Symbol continua diferente e registrado.

Os outros 92 casos conservam exatamente stdout, stderr e código de saída dentro do cristalino; o oráculo também permaneceu idêntico. Há 39 diferenças de diagnóstico anteriores restantes neste corpus; JSON/fields, reflexão, síntese e medidas relativas continuam sem certificação global de paridade. Consulte observables-result.json para cada caso.

**Suíte completa: 8836 aprovados, zero falhas e sete ignorados.** Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices, V1/V5/V15/V26 estrito, reverse lineage e diff passaram. Resselo restrito aos doze owners após V15/V26 válido. Nenhum novo erro/warning; todos os diagnósticos fora dos doze consumers permanecem idênticos. SARIF: **{'V16:error': 117, 'V19:note': 1076, 'V20:note': 873}**. O gate global continua exit 1 pelos 117 V16 restantes.

A projeção dos campos já transportados pela caption também corrige body/position/separator conhecidos, callback nativo refletido, metadados da função de elemento, JSON/YAML CLI e json.encode. As 15 sondas adicionais têm 11 valores completos coincidentes com o pin e 4 limites explícitos em caption-shape-observables-result.json; os limites não contam como paridade aprovada. O constructor qualificado aceita as coerções de conteúdo medidas, sem representar separator:auto com um sentinel.

## Limites preservados e próximos passos

**FG1 permanece incompleto:** FigureElem.kind:Option<String> não distingue uma função de elemento da string homônima. Funções de elemento válidas continuam no caminho histórico; sua identidade, síntese, presença de caption omitida/none, metadados de position/separator, contadores e seletores exigem revisão pública própria. A inferência atual também não percorre todos os wrappers/containers. Não foi movida de fase nem declarada correta nesta etapa.

**SY1 permanece incompleto:** o modelo de Content ainda não distingue o carrier SymbolElem. Preservar o texto não prova igualdade de função/reflexão/morfologia com o pin.

**RM1 permanece incompleto:** extract_length ainda reduz Relative à parte absoluta e Ratio a zero; números são aceitos em alguns callers onde o pin rejeita. Essa aceitação consta de contratos históricos e não é automaticamente um defeito: RM1 deverá distinguir extensão intencional de perda de informação antes de mudar compatibilidade. Spacing e dimensões de Box possuem carriers públicos de Length; Block já conserva Rel e não foi regredido. A correção completa exige medir cada consumer e preparar contratos, migrações e testes antes do gate ADR-0127. A aprovação TS1 não cobre essas alterações.

[FG1/RM1](follow-ups.md) nomeiam os passos que completam os subconjuntos; os L0 declaram a incompletude. contract-dependencies-before.json registra a busca lexical de dependências, incluindo testes, como ponto de partida, não como lista completa de migração autorizada. RG1 exige rebase sobre a árvore atual, inclusive as novas delegações de Content::get_field e a projeção CLI; não sobrescrever por candidatos antigos. O novo núcleo caption-projection compartilha quatro invariantes entre oito owners com pins completos; código continua apontando somente aos seus owners 1:1. As 18 diferenças de diagnóstico do lote de catálogos continuam independentes, assim como RG1 pendente, Paint textual, seletor show text e demais débitos em pendencias.json. [V16 restantes](inventario-v16-restante.md).

## RAM e proveniência

Comandos pesados sequenciais sob **12 GiB rígidos, high 10 GiB, swap zero e Cargo jobs=1**. Pico **10739572736 bytes (10.002 GiB)**; eventos agregados de execuções únicas {'high': 105, 'max': 0, 'oom': 0, 'oom_kill': 0}. Sem OOM/OOM-kill. Memória de cgroup inclui cache e não é RSS nem prova de otimização.

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, working tree não commitado. before/final-state.json.gz, logs, RED/GREEN, recursos e corpora fixam UTC, status/diffstat, fontes/L0/config/binários e comandos. [Patch desta etapa](patch.diff). Mudaram doze consumers, seus doze L0 e o novo núcleo caption-projection desde o checkpoint anterior; TS1, igualdade Styled e lote de catálogos foram preservados por hash. Manifestos TS1/RG1 conferidos integralmente e preservados.

Vanilla pin **9f2b6e8715237cb086899a42873660fe744622e8**, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter pin **12797073972e375d9ab078cf6750c8b30b71e2b9**, binário SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2. Fluxo contínuo ADR-0127 para as correções internas entregues; nenhuma aprovação de FG1/RM1 foi inferida.
