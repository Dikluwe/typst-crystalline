# L0 — infra/system-world/font_session
Hash do Código: 2c6999de

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:391cd1604976262b1b6e019b802ed72e78782260d2788bb04999297559605509

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/font_session.rs`


## Medição anterior à decisão de retenção ratificada

00_nucleo/diagnosticos/watch-retencao-2026-10-07.md registra HEAD/árvore/binários/UTC,
edições reais, recuperação e contagem dinâmica de aquisição por ciclo.
00_nucleo/diagnosticos/watch-validacao-2026-10-07/measurement.json mede validação por
descoberta versus aquisição completa no mesmo estado; controls.json testa adição,
remoção, substituição, mtime restaurado, links e configuração. São custos isolados,
não ganho de CLI ou prova de intenção upstream. FontBook/slots/cobertura devem
preservar seleção, conteúdo, diagnóstico e render na linguagem; duração/Arc são
mecânica. Inferimos benefício possível, refutado por custo, stale resources,
regressão ou retenção ilimitada. Fixtures/proveniência/limites constam do relatório.

### Decisão ratificada sob ADR-0127 §2.1/§2.2

Dono ratificou com Confirmo o pacote 00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json. Receipt: 00_nucleo/diagnosticos/watch-catalogo-execucao-2026-10-07/ratification.json. Cada owner continua 1:1.

## Contrato público e propriedade

Tipo público opaco FontCatalogSession, sem campos públicos:

```rust
impl FontCatalogSession { pub fn new() -> Self; }
```

new não faz I/O. Sessão L3 possui Mutex de Option de uma única geração contendo
key e Arc do par slots/book. Nenhum global, persistência, novo trait ou unsafe.
A operação de catálogo é pub(super), exclusiva dos descendentes World.

Sob a serialização da sessão, obter prova atual elegível ou preparar descoberta
atual via owner generation.
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

Na configuração elegível descrita em `infra/fontdb/generation.md` §Prova
rápida estrita, a sessão pode comparar uma prova atual completa com a prova
guardada junto da geração, verificando configuração e identidades retidas sem
novo `read_dir` de fontes. Hit validado devolve o Arc pareado sem chamar
prepare; qualquer diferença, recusa ou instabilidade segue exatamente o
caminho fresco acima. Ao preparar, colher uma prova completa antes da descoberta,
revalidá-la após construir o catálogo e guardar apenas uma prova estável para
eventual próximo hit.
A prova não guarda faces/slots separadamente,
não amplia o lifetime da geração, não publica API nova e não altera a demanda
lazy nem o reconhecimento de recursos na próxima recompilação.

Testes RED→GREEN inline: mesma geração evita reconstrução (contador privado de
teste); key alterada/recusada e postcheck falho impedem reuso; múltiplos Worlds e
configurações concorrentes mantêm book/slots associados e fontes corretas;
alterar/remove/criar arquivos e falhas negativas não herda Sources/read_cache.
Use assert de identidade apenas para observar reuso interno, nunca como gate de
paridade. Aceitação do produto é seleção, texto, diagnósticos e render.


## Expressões equivalentes e CI estrito — 2026-10-14

### Medição anterior à decisão

`03_infra/src/world/font_session.rs` nas linhas [55, 73, 79] contém as expressões apontadas pelo inventário
Clippy. `00_nucleo/diagnosticos/ci-simplificacoes-2026-10-14/before.json.gz`
conserva fonte/owner, spans, HEAD, working tree não commitado, UTC e diffstat;
red.log registra os lints selecionados como erros antes de alterar código.
Os tipos e usos foram conferidos no consumer e no toolchain 1.92.0 pinado.
Esta é manutenção da expressão Rust; não é medição nova de paridade upstream.

### Responsabilidade e aceitação

Em catalog_with_fast, passar o callable verify diretamente aos predicados Option; emprestar &verify nos usos anteriores e consumi-lo somente no último uso. Preservar chamadas e sua ordem condicional, curto-circuito, lock, postcheck, invalidação, fallback e geração única. A remoção de closures identidade não autoriza executar verify antecipadamente ou cachear seu resultado.

Correção interna em fluxo contínuo ADR-0127, sem API pública, default, fase,
unsafe, dependência ou supressão de lint novos. L0 precede código e resselo
fica restrito ao owner. Usar os controles funcionais existentes; não alterar
fixtures, expectativas, tolerâncias ou registrar testes que apenas espelhem
esta simplificação. Os lints selecionados devem passar como erros, seguidos
de suíte completa, build, all-targets, rustdoc, fmt, linhagem e inventário total.


## Auxiliares de teste e bindings produtivos — 2026-10-14

### Medição anterior à decisão

`03_infra/src/world/font_session.rs` nas linhas [35] apresenta os avisos selecionados.
`00_nucleo/diagnosticos/ci-helpers-testes-2026-10-14/before.json.gz`
conserva fonte/owner, HEAD, working tree não commitado, UTC e diffstat.
references-before.json.gz registra chamadas, imports, módulos e referências;
o estado da suíte/inventário anterior foi verificado por hashes. A lista de
testes foi capturada antes da edição. Não se infere inutilidade funcional
apenas de dead_code: as chamadas privadas e sua compilação foram auditadas.

### Responsabilidade e aceitação

catalog_with é adaptador privado sem prova rápida, chamado apenas pelo módulo cfg(all(test, unix)). Aplicar a mesma condição à definição, mantendo seu corpo e todos os controles de fallback/poison/concorrência. catalog e catalog_with_fast continuam produtivos e inalterados. O helper não integra a API pública de FontCatalogSession.

Manutenção interna em fluxo contínuo ADR-0127: nenhuma API pública, campo,
default de produto, fase, dependência ou supressão de lint muda. O gate de
diagnósticos selecionados deve falhar antes e passar depois; ele não é um
novo teste funcional nem prova de paridade. Preservar a lista e as expectativas
dos testes existentes, executar a suíte completa, compilar lib/all-targets,
verificar build/rustdoc/fmt/linhagem e registrar o inventário total. Não criar
testes que apenas espelhem cfg ou remoção de bindings. Resselo restrito ao owner.


## Expressões locais verificadas — Clippy 2026-10-16

### Medição anterior à decisão

03_infra/src/world/font_session.rs:240 (clippy::len_zero). O snapshot `00_nucleo/diagnosticos/clippy-expressions-2026-10-16/before.json.gz` conserva fontes/L0, HEAD, working tree não commitado, status, diffstat e UTC; initial-diagnostics.json conserva sugestões e spans. Os usos e tipos foram inspecionados antes da decisão. São representações Rust internas; nenhuma nova paridade upstream ou ganho de performance é alegado.

### Responsabilidade exclusiva e aceitação

No teste de aquisição preguiçosa, usar !world.book().is_empty() em vez de len > 0, mantendo ordem das consultas/assertions e verificação de cache. Não alterar new(), Default, aquisição ou política de sessão.

Fluxo contínuo ADR-0127: sem API pública, default de produto, fase, dependência ou supressão novos. L0 precede código e resselo restrito. Preservar inputs e resultados dos controles existentes, sem relaxar tolerâncias. Gate dos lints selecionados RED→GREEN; executar suíte workspace, build, Clippy all-targets, formato, Rustdoc e linhagem. Não criar testes que apenas espelhem a sintaxe Rust.


## Default público — proposta sujeita à confirmação ADR-0127

### Medição anterior à decisão

`03_infra/src/world/font_session.rs:23` declara pub fn new() -> Self sem argumentos. O inventário em `00_nucleo/diagnosticos/field-reassign-2026-10-15/clippy-inventory.json` registra new_without_default; fonte/L0/hashes atuais são preservados no pacote clippy-default-2026-10-16. FontCatalogSession::new cria Mutex::new(None) por instância, sem descoberta ou leitura de fontes. Default cria sessão independente, vazia e lazy, delegando a Self::new(); catálogo e política de reuso permanecem inalterados. Isto é contrato Rust de construção, sem nova medição de paridade da linguagem.

### Contrato proposto

Adicionar somente `impl std::default::Default for FontCatalogSession` com `fn default() -> Self` que devolve `Self::new()`. Preservar integralmente new(), sua visibilidade, campos, seus valores, todos os demais métodos e semântica. Não criar outro trait, novo modo/flag, estado compartilhado global, I/O no construtor ou mudança de fase. Nenhum caller existente precisa migrar; novo código pode usar `<FontCatalogSession as Default>::default()` e bounds `T: Default`.

### Gate, testes e aceitação

Nenhuma materialização antes da confirmação humana do manifesto deste pacote (ADR-0127 §2.1/§2.4). Os impls são acréscimos públicos: sua remoção futura quebraria consumidores do novo contrato. Após ratificação, a cláusula só supersede proibições genéricas de nova API para estes três impls expressos; os demais contratos permanecem. Testes de uso por bound Default devem falhar por ausência de implementação antes do código e passar após delegação. Verificar valores/isolamento de contextos e sessões novas, preguiça de recursos e conservação dos controles existentes; suíte workspace, Clippy all-targets, build, Rustdoc, formato e linhagem com resselo restrito. Não alterar valores/defaults nem suprimir new_without_default.
