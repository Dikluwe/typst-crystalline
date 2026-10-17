# L0 — infra/system-world/font_session
Hash do Código: PENDENTE

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/font_session.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509

## Medição anterior à decisão de retenção (candidato não ratificado)

00_nucleo/diagnosticos/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
00_nucleo/diagnosticos/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão proposta sob ADR-0127 §2.1/§2.2

Aguardar ratificação do pacote 00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json
antes de instalar este candidato ou escrever código produtivo. Cada owner continua 1:1.

## Contrato público e propriedade

Tipo público opaco FontCatalogSession, sem campos públicos:

```rust
impl FontCatalogSession { pub fn new() -> Self; }
```

new não faz I/O. Sessão L3 possui Mutex de Option de uma única geração contendo
key e Arc do par slots/book. Nenhum global, persistência, novo trait ou unsafe.
A operação de catálogo é pub(super), exclusiva dos descendentes World.

Sob a serialização da sessão, preparar descoberta atual via owner generation.
Se key coincide e a rechecagem das identidades tem sucesso, clonar Arc da geração;
não reconstruir metadados/cobertura nem FontSlots. Se mudou, construir o catálogo
a partir da descoberta preparada e revalidar identidades antes de guardá-lo.
Não guardar geração instável. Recusa/erro/poison remove ou desconsidera a geração
antiga e executa aquisição combinada fresca do owner fonts, sem cachear o fallback.
Não expor erro novo de usuário nem reutilizar resultados negativos antigos.

A sessão conserva somente uma geração para reuso futuro. Clones de Arc em Worlds
em uso ficam válidos por ownership Rust; não alterar seus índices ou limpar seus
slots. Os mapas por arquivo e todos os outros serviços permanecem no World novo.
Guardas de validação e descoberta estão em generation; composição está em fonts.

Testes RED→GREEN inline: mesma geração evita reconstrução (contador privado de
teste); key alterada/recusada e postcheck falho impedem reuso; múltiplos Worlds e
configurações concorrentes mantêm book/slots associados e fontes corretas;
alterar/remove/criar arquivos e falhas negativas não herda Sources/read_cache.
Use assert de identidade apenas para observar reuso interno, nunca como gate de
paridade. Aceitação do produto é seleção, texto, diagnósticos e render.
