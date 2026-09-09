# P1322 — decisão causal provisória C (não é seleção final)

Regime: executado sem atestação técnica de isolamento. Sem aprovação pelo próprio classificador. Esta nota antecede o fim da matriz reversa e a revisão D; não autoriza implementação nem encerra P1322.

## Medição antes da decisão

O recibo `p1322-sentinels-normal.json`, SHA-256 `c3d937946149b9948b8c91999599e1d2b726ed7cba0b0a5aec9c303a780d8d34`, mede `assert.nope`, `json.nope`, `yaml.nope`, `toml.nope`, `cbor.nope`, `table.nope` e `json.with().nope`. Vanilla nomeia a função pública, usa backticks e marca somente o campo; cristalino omite o nome, usa aspas e marca o acesso inteiro. Cada rota foi observada nos quatro perfis. O alias With de json é controle causal, não sétimo path.

Fonte atual: `01_core/src/compiler/eval/bindings/field_access.rs:181` restringe o helper de nome a nativas sem namespace; `:271` exclui Some do field-only span; `:420` contém o erro genérico de namespace Some. `01_core/src/entities/func.rs:291` já fornece o nome e `:355` delega namespace por With. Vanilla `foundations/func.rs:291` consulta scope e forma o erro nomeado, e `typst-eval/src/code.rs:347` usa o span do campo. L0 proprietário `00_nucleo/prompts/compiler/eval/bindings/field_access.md:170` preserva explicitamente esta categoria fora de P1311. Portanto isto é dívida diagnóstica conhecida, não contradição específica do L0 nem regressão P1311.

## Hipótese delimitada e refutável

Coorte `namespace-function-missing-field`: erro de campo ausente de Native/NativeWithEngine com namespace Some, inclusive With, com lookup presente preservado. Nome nativo subjacente e field span já existem neste owner; não é necessário mudar Func, Args, dispatch, pipeline ou render para este observável. Um nome nativo cujo carrier atual não identifique a identidade pública, uma origem perdida antes deste owner ou necessidade de outro consumer refuta a suficiência de owner único e exige reabrir a hipótese.

Possível gate: correção interna de paridade ADR-0127, L0-first, RED→GREEN e revalidação. O L0 futuro deve substituir a preservação Some somente nesta fronteira; não generalizar a Closure/Plugin/Element de usuário, None, Module/Dict/Type/Content/Float, nem criar namespace/membro. Risco demonstrado delimitado: branch de lookup ausente + escolha de span na categoria; controles de sucesso json/yaml/toml/cbor/table e proteção None já estão no suplemento. Os ataques futuros precisam incluir alias, With encadeado, nome qualificado, campo como callee, multilinha e preservações adjacentes; esta auditoria não executa a correção.

## Comparação provisória

Sem regressão nova de classe nos 2182 IDs históricos da matriz normal; três acessos encode bilateralmente ausentes passaram a diagnóstico igual. Isso não certifica chamadas ausentes do corpus. IDs novos não recebem rótulo de regressão.

A coorte Some tem prioridade 3, um owner completo e seis paths. O warning do ancestor math.join tem quatro paths (binding e três continuidades), um owner; o span não-Module de Dict/Content/Float tem três categorias medidas e um owner. CBOR não conserva a antiga alegação barata de owner único: as rotas indiretas expõem também nome `cbor.encode` no trace, registrado em `compiler/eval/mod.rs:1836` e lido em `call_dispatch.rs:1648`. Faltantes futuros e causas de origem externa/captura permanecem com risco/owner set desconhecidos, não elegíveis por contagem de glue.

Na matriz normal ampliada, modifiers gt/lt.tri e tack.double mostram depreciação ausente; `sym.md:90` individualiza essa divergência intencional e `entities/symbol.md:165` a mantém fora do carrier. Não é crédito de paridade nem extensão. Quote single/double compartilha a causa `entities/symbol.rs:121`: escape_debug seguido de escape de aspas.

Pendências para seleção final: reversa completa; validação da matriz transversal R2 com flags reais nos quatro perfis; causalidade de qualquer diferença adicional R2; reconciliação e revisão D dos artefatos finais. Dívida F/S/A permanece em artefato separado, sem execução de mutantes de produto nem quitação por P1308.

Nota de correção do auditor: suspeita inicial de escapes duplicados nos boundary-0/1/2 foi refutada pela inspeção de caracteres. As strings R1/R2 são estritamente idênticas e usam escape único no texto Typst; a repetição redundante não aumenta cobertura nem denominador.
