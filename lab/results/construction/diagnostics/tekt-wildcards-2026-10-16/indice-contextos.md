# Índice de contextos V16 — próxima auditoria

Extração somente de leitura após o lote representação/identidade. **312 avisos emitidos + dois locais preservados sem emissão = 314 contextos**, em **136 owners**. Não é L0 ou prescrição de implementação; nenhum código/L0 mudou nesta extração.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Extração 2026-10-16T19:51:57.959966+00:00–2026-10-16T19:52:00.556106+00:00 UTC. inventory-details.json conserva hashes globais da validação final, origem SARIF e seu SHA-256, helper/rlibs/comandos e prova de fontes inalteradas. contexts.json associa cada local à função, scrutinee e match completos por AST; owners-context.json.gz guarda os L0. Os locais sem emissão são T027 e T158/T160, preservados de V17.

## Prefixos observados nos padrões

A coluna é uma inferência sintática do prefixo qualificado mais frequente, **não resolução de tipo Rust**. Aliases como FI e Self exigem leitura dos imports/impl; unresolved-type exige classificação manual. Nenhuma correção pode partir apenas desta contagem.

| Prefixo | Locais |
|---|---:|
| Content | 154 |
| FrameItem | 67 |
| Value | 43 |
| unresolved-type | 8 |
| Selector | 7 |
| SyntaxKind | 5 |
| Type | 4 |
| FI | 3 |
| ElementKind | 2 |
| Expr | 2 |
| FlowEvent | 2 |
| HAlign | 2 |
| MathSize | 2 |
| SpaceSupport | 2 |
| AttachedFileRelationship | 1 |
| BlockHeight | 1 |
| ColorSpace | 1 |
| CounterKey | 1 |
| ElementPayload | 1 |
| Format | 1 |
| MathStyleKind | 1 |
| NodeKind | 1 |
| P1293HtmlNative | 1 |
| SemanticKind | 1 |
| TextEdge | 1 |

Primeira divisão sugerida para análise: Content/FrameItem concentram o volume; Value e SyntaxKind requerem auditar os casters e complementos, sem trocar matches por if-let para ocultar V16. Priorizar grupos por contrato/owner, com L0 vigente lido antes de qualquer proposta. Não construir classifiers, APIs ou listas nominais a partir de prefixos sem confirmar enum, cardinalidade, guards, efeitos e fallbacks. A camada e o owner permanecem a unidade de implementação.

O recorte de MathStyleKind da pré-análise histórica foi concluído no lote tekt-enums-2026-10-16; seu relatório registra 21 matches e 11 avisos corrigidos em sete owners. O novo snapshot de implementação deve partir de final-state.json desse lote, sem reutilizar o before antigo como estado corrente.
