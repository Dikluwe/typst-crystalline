# 💎 Semente (Spec L0) - Analyze IDE (typst-ide/src/analyze.rs)

## 1. Objetivo Central
O módulo `analyze.rs` atua como o motor de inferência estática e dinâmica para recursos de IDE (LSP, tooltips, autocompletion). Ele permite que o sistema de desenvolvimento inspecione a Árvore de Sintaxe (AST) através de `LinkedNode` para extrair os valores matemáticos avaliados das expressões (`Value`), tentar mapear _imports_ pre-compilados, e raspar um documento inteiro já diagramado em busca de identificadores (`Labels` e chaves de Bibliografia).

## 2. Atomização da Lógica Pura (Para o futuro L1)
O arquivo atual acopla inferência sintática a chamadas engatilhadas de Engine pesada. Eis os algoritmos que devem ser purificados:

* **`analyze_basic_expr`** (Desvencilhado de `analyze_expr`): Lógica atômica e puramente matemática/sintática que mapeia Literais da AST (`None`, `Auto`, `Bool`, `Int`, `Float`, `Numeric`, `Str`) em seus correspondentes Enum `Value` legados do Typst, retornando *early* sem acionar compilação. Além disso, implementa as regras recursivas de travessia *UP* e *DOWN* (como olhar o nó *Contextual* filho ou o paí de um *FieldAccess*).
* **`analyze_scope_fallback`** (Baseado em `analyze_expr_with_fallback`): Lógica de busca de escopo best-effort usada em *dead code*. Recebe um Identificador e um mapa restrito de Globals injetados (`crate::utils::globals`), fazendo navegação encadeada no dicionário estático (Dict/Scope lookup em structs de FieldAccess) sem ter acesso direto à compilação principal.
* **`extract_document_labels`** (Baseado em `analyze_labels`): Função puramente funcional (redução/filtro) que não re-compila nada. Ela varre um grafo de conteúdo prévio (`introspector.all()`), deduplica labels únicos de visualização (`FxHashSet`), e aplica regras pesadas de "desempacotamento" (unpack) tentando extrair uma legenda (`Caption` em figurar) ou texto limpo (`body.plain_text()`). No final, concatena tudo junto com o `BibliographyElem::keys`. Lógica 100% pura para L1.

## 3. Efeitos Colaterais Identificados (Para os futuros Contratos L3 / Orquestrador L2)
O arquivo parece "simples", mas possui dependências perigosas e chamadas atreladas a IO disfarçadas usando o closure de Engine do Typst:

* **Efeito 1: Disparo de "Tracing" no Compilador (CPU/Memory Impureza)**: 
  Quando `analyze_expr` não consegue resolver um nó estático simples, ele faz Fallback para rodar a Orquestração do Compilador em modo rastreável (`typst::trace::<PagedDocument>(world.upcast(), node.span())`). Isso significa que a IDE força o disparo total do pipeline legado (que contém System IO, Time, Memória, Cache `comemo`) a partir de um único clique num literal na IDE.
* **Efeito 2: File System / Rede Disfarçada no Import Lookup**:
  A função `analyze_import` empacota uma invocação à avaliadora com a Engine mutável: `typst_eval::import(engine, &path, source_span)`. Isso delega para o núcleo a leitura real de `.typ` no disco (FileSystem Read) ou disparo em requisições de Pacote HTTP (Package Download IO). O fato de uma ferramenta do IDE de "Inspeção" chamar o executor de imports mascara o efeito colateral sob `IdeWorld`.
* **Efeito 3: Resolução de Escopo Global via Biblioteca (`utils::globals`):**
  Função privada no legado (`mod utils;`). Detecta se o cursor está em contexto Matemático (`SyntaxKind::Equation|Math|MathFrac|MathAttach`) ou Global, e retorna o `Scope` correspondente via `world.library().math.scope()` ou `world.library().global.scope()`. É a ponte entre o `IdeWorld` e o dicionário de símbolos disponíveis. **Deve ser abstraída num Contrato L0 (`IIdeEnv`) para injeção.**
* **Efeito 4: Fábrica de Engine Efêmera (`utils::with_engine`):**
  Função privada no legado que constrói uma `Engine` temporária com `Introspector::default()`, `Traced::default()`, `Sink::new()`, `Route::default()` e `world.upcast().track()`. Usada por `analyze_import` para executar imports. **Deve ser abstraída no mesmo Contrato L0 (`IIdeEnv`).**

*(Os contratos para suportar as delegações destes I/Os devem ser formalizados em `00_nucleo/contracts/ide_env.rs`, com implementação concreta em L3).*

## 4. Glossário / Assinaturas (Estruturas de Dados)

* **`LinkedNode`**: Representa um ponteiro inteligente na estrutura de código fonte AST do Typst. É a moeda de troca universal na inspeção do IDE, fornecendo percurso parental entre *tokens*.
* **`Value`**: O polimorfismo supremo da VM do Typst, representando os resultados de variáveis executadas.
* **`AsDocument`**: Uma forma de garantir um artefato empaginado com um Introspector (Árvore resolvida pós-layout).
* **`IdeWorld`**: Trait ampliada do `World` que acopla estado do projeto ativo do usuário no Editor aos mecanismos do legado.
* **`BibliographyElem`, `FigureElem`**: Elementos nativos construídos no layout que devem ser polimorficamente consultados para raspar os Labels.
