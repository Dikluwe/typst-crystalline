# Prompt L0 — entities/module
Hash do Código: 72c197ba

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:2b8888460a1e6008abda0ff71603fe4dace79ff0a2a7aad598f8aabfab3e3776

**Passo**: P536
**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/module.rs`
**ADRs relevantes**: ADR-0017 (adiamento eval/typst-library), ADR-0023 (indexmap/Scope)

## Contexto

`Module` é o resultado de avaliar um ficheiro Typst. Contém um `Scope`
de bindings e um nome. O original usa `Arc<ModuleInner>` — replicado
aqui porque módulos são clonados entre ramos de `eval()` e `Arc` torna
o clone O(1) em vez de O(n).

Content já é transportado em campos opcionais, conforme o contrato vigente abaixo. A omissão de Content do passo inicial é histórica e não se aplica ao consumer atual.

`EcoString` substituído por `String` (ADR-0004 Opção C / padrão do projecto).
`typst_syntax::FileId` substituído por `crate::entities::file_id::FileId`.

## Interface pública

```rust
pub struct Module(Arc<ModuleInner>);

impl Module {
    pub fn new(name: impl Into<String>, scope: Scope) -> Self
    pub fn name(&self) -> &str
    pub fn scope(&self) -> &Scope
    pub fn realization_introspector(&self) -> Option<&TagIntrospector>
    pub fn document_info(&self) -> &DocumentInfo
    pub fn set_document_info(&mut self, info: DocumentInfo)
}

impl Clone for Module  // O(1) — Arc::clone
```

## Campos do `ModuleInner`

```rust
struct ModuleInner {
    name: String,
    scope: Scope,
    content: Option<Content>,
    introspection_content: Option<Content>,
    realization_introspector: Option<Arc<TagIntrospector>>,
    bib_styles: HashMap<u64, Arc<IndependentStyle>>,
    document_info: DocumentInfo,
}
```

- `content` — output renderizado pós-show-rules (usado por layout/PDF).
- `introspection_content` — árvore semântica realizada com proveniência de origens e produtos, usada
  para construir o `TagIntrospector`. Garante que `query(heading)` encontra
  o elemento mesmo quando uma show-rule o transforma no output renderizado.
- `bib_styles` — styles CSL resolvidos em eval time (P429 / DEBT-63).
- `document_info` — metadados do documento definidos por `#set document(...)`
  (P536). Transporte eval → pipeline → exportador PDF (`/Info`).

## Critérios de Verificação

```
Dado Module::new("my-file", scope_com_x)
Quando name() e scope().get("x") forem chamados
Então "my-file" e Some(&Value(()))

Dado Module criado e clonado
Quando ambos são usados
Então clone é consistente (mesmo nome)

Dado Module::new("empty", Scope::new())
Então scope().is_empty() = true
```


## Medição e atualização da realização

`01_core/src/entities/module.rs:39,97–104` já transporta introspection_content. O eval antigo de `01_core/src/compiler/eval/mod.rs:872–889` preenchia o campo por segunda execução do programa. O gate de realização única revoga essa mecânica: content conserva o visual associado à proveniência e introspection_content conserva o grafo semântico realizado, incluindo origens consumidas e produtos. Nenhuma assinatura ou campo muda; não escolher árvores por quantidade nem exigir reavaliação para preencher o carrier.


## Memoização por módulo avaliado — 2026-10-05

### Evidência anterior à decisão

No HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree da avaliação única, `01_core/src/compiler/realize/mod.rs:197–200` constrói o TagIntrospector. `01_core/src/compiler/eval/mod.rs:888–896` descarta esse resultado e publica só o Content. `03_infra/src/pipeline/paged.rs:242`, `03_infra/src/pipeline/html.rs:102` e `03_infra/src/query_helpers.rs:478,519` reconstruem o índice inicial. Fontes e proveniência da implementação anterior estão em `lab/results/construction/diagnostics/dupla-avaliacao-implementacao-2026-10-05.md` e seus recibos. O índice calculado não possui posições ou páginas finais. Isso comprova trabalho redundante, não um ganho ainda medido.

`01_core/src/entities/content/equality.rs` e `01_core/src/entities/content/rust_hash.rs` projetam Realized no visual: a igualdade ou hash genérico de Content não é chave suficiente para distinguir origens, Locations e proveniência. A escolha de chave a seguir é mecânica interna; a aceitação permanece nas consultas, identidades, ordem causal, contadores, contextos e geometria.

### Contrato e fronteira

ModuleInner pode transportar `realization_introspector: Option<Arc<TagIntrospector>>`, privado e inicialmente ausente. Acrescentar o getter público `pub fn realization_introspector(&self) -> Option<&TagIntrospector>`. O setter interno é `pub(crate) fn set_realization_introspector(&mut self, introspector: TagIntrospector)`; respeita o mesmo requisito de Arc único dos mutadores existentes. TagIntrospector usa o caminho canônico `crate::entities::introspector::TagIntrospector`.

Esse índice é snapshot semântico da própria avaliação/realização e do conteúdo final publicado no Module, não um cache global, por texto, por endereço cru, entre arquivos ou entre Worlds. A identidade do Module é a chave implícita. O Arc conserva snapshot e lifetime e mantém clone de Module O(1). Sem unsafe, I/O, relógio ou estado global. O owner não altera a igualdade/hash de Content nem de Module.

`set_content` e `set_introspection_content`, quando conseguem efetivamente alterar o ModuleInner único, invalidam o índice antes de publicar o novo conteúdo. Não reutilizar snapshot de outra árvore; não depender da cardinalidade. Clones continuam imutáveis segundo o contrato já vigente. Publicar o índice somente depois de todos os ajustes do Content e dos setters correspondentes. Módulos sem realização mantêm None e os callers usam o caminho anterior.

O snapshot não recebe posições, páginas, callbacks executados em tentativas posteriores, estilos bibliográficos injetados pelo coordenador ou conteúdo expandido depois. O coordenador clona o snapshot antes de mutar stores para uma tentativa; nunca modifica o Arc armazenado. Alteração de metadata/bib_styles não torna o índice geométrico: cada caller mantém suas injeções vigentes na cópia independente.

### Aceitação e limites

Exigir RED→GREEN para módulo com show que disponibiliza índice contendo original e gerados; invalidar após mudança de qualquer carrier de conteúdo; clone consistente e sem contaminação pelo índice de uma tentativa posterior; campos/labels/ordem causal iguais a reconstrução fresca. Revalidar PDF, HTML, query e bundle. Bundle conserva o caminho próprio de sementes e resets por documento; este getter não autoriza trocar esse algoritmo pelo índice do root.

Medir antes/depois contra o executável preservado da avaliação única, com fontes, hashes, estado da árvore, horários, tempo e memória. Não alegar benefício universal nem memoização de layout/células. A nova fronteira pública requer o gate ADR-0127 antes da materialização; os demais owners são atualizados antes de suas respectivas alterações.


## Gate HTML e identidade de módulos — 2026-10-17

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/html-feature-identity-2026-10-17/before.json.gz registra
UTC, status, diff HEAD --stat, fontes e L0 completos e hashes; todos os 2123
hashes da etapa anterior foram conferidos antes de reutilizar sua matriz.
before-probe.json.gz conserva argv, fixtures, outputs e SHA-256 dos binários.
O vanilla é o pin 9f2b6e8715237cb086899a42873660fe744622e8 e o binário
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
fields.rs:39–52 só aplica o gate html a format, não a std; identity.rs:84–101
reconhece módulos por nomes/bindings. As sondas com import correto refutam
paridade: std.html direto/alias/closure/reexport aceita Html desligada;
global.typ com type/format/document e asset comum recebe gate indevido;
format.typ com funções svg/png/pdf/html recebe gates html/bundle indevidos.
A tentativa inicial de sonda com let m = import foi inválida (import retorna
none) e está arquivada em candidate-probe-syntax; não conta como evidência.
A fonte upstream pinada module.rs:138–154 lê o guard do binding e code.rs:
347–368 ancora no field. Isto comprova semântica/diagnóstico, não exige
replicar BindingGuard ou endereços upstream. A instrução vigente no L0 pai
field_access.md:61–62 proíbe identificar std comparando bindings.

### Decisão e responsabilidade exclusiva

Adicionar somente enum privado ModuleKind { Ordinary, Std, Format } e campo privado kind em ModuleInner. Module::new continua público e delega à construção privada com Ordinary, mesmo com nome global/format e bindings idênticos. Adicionar factories pub(crate) Module::stdlib(scope) e Module::format(scope) com nomes global/format fixos e predicados pub(crate) is_std/is_format. Não mudar debug nem Eq/Clone, setters, conteúdo, cache, metadata ou bibliografia. Medir sizeof de Module/ModuleInner antes/depois sem prometer ganho.

Consumir o núcleo canônico pinado builtin-module-identity.toml. O marcador é
mecânica privada para cumprir gates vigentes, não novo contrato público;
ADR-0127 permite fluxo contínuo de correção/paridade. A API pública de Module
continua opaca, sem novos campos/métodos exportados; comparar declarações e
assinaturas antes/depois. Nenhuma alteração pública/fase/default é autorizada.

### Aceitação e limites

RED funcional antes da implementação; GREEN dos gates, homônimos, aliases,
closures e imports nos oito perfis; mensagem/hints/span e precedência causal.
Não atribuir crédito MATCH/Preserved de feature ativa a casos desligados:
classificar separadamente rejeição de feature e aceitação com feature ativa.
Conservar divergência prévia de igualdade entre std de arquivos distintos;
esta etapa não altera Eq de Module nem declara paridade global.
Testar a matriz anterior de caminhos std.html desligados e controles ativados;
sem publicar, commit ou release. Build, suíte workspace, Clippy all-targets e
Rustdoc estritos, fmt, V1/V5/V15/V26, reverse e diferencial global sem novo
erro/aviso. Somente resselo dos seis owners após validar ownership/núcleos.
Comandos pesados sequenciais com MemoryMax12GiB/High10GiB/swap0/jobs1.


## Continuação causal do gate — chamadas e set rules

Medição anterior à decisão: candidate-1/validation-green.log registra quatro
controles aprovados e falha real do teste de argumentos: std.html(panic(...))
executa panic antes do gate. before-set-probe.json.gz registra UTC, hashes,
fontes e saídas bilaterais no mesmo HEAD/árvore não commitada: std.html,
format.html, std.asset e format.bundle chamados avaliam argumentos antes do
gate; set std.html/std.format.html/alias apaga o erro do target e ignora os
argumentos. Variável/field ausente em set também é aceito indevidamente;
condition false conserva atalhamento. Html ativo em set por alias ignora panic
no Tekt e o executa no pin. São ordem causal, diagnóstico e efeito da linguagem.
collection_methods.rs:36 avalia argumentos antes de autenticar módulo; set.rs:
44 usa ok() na resolução e :326 autentica HTML pela sintaxe literal.

Conservar a responsabilidade anterior; fields extrai os três gates para check_module_field_feature(&Value, &str, Span, Features) -> SourceResult<()>, interno a eval, e o chama antes do lookup. Os demais owners não duplicam os diagnósticos nem regras.

L0 precede esta continuação; correção interna/paridade em fluxo ADR-0127.
O núcleo builtin-module-identity continua pinado e sem alteração. O contrato
público e os campos externos permanecem iguais; nenhum fallback novo ou
supressão. Os controles e a matriz dos oito perfis permanecem; acrescentar
verificação bilateral dos efeitos e de set com/sem feature. A suite completa
regressa após o source freeze novo, sem reaproveitar gates de fonte alterada.
