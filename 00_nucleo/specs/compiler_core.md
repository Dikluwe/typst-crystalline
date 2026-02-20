# 💎 Semente (Spec L0) - Compiler Core (typst/src/lib.rs)

## 1. Objetivo Central
Atuar como a principal interface (ponto de entrada) do compilador Typst. Este arquivo orquestra todo o ciclo de vida do documento: Parser -> Evaluation -> Layouting -> Exporting. Em especial, seu papel lógico principal é o "laço principal" (main loop) de compilação iterativa, que realiza as execuções repetidas da marcação para estabilizar as "introspecções" (como numeração de páginas, índices e posições físicas de elementos).

## 2. Atomização da Lógica Pura (Para o futuro L1)
As funções atômicas e algoritmos puros extraídos deste módulo legado são:

* **`orchestrate_compilation_loop`** (Baseado em `compile_impl`): A lógica de controle de fluxo de layout iterativo. Recebe uma árvore de conteúdo purificada e executa as passagens repetidas (até um limite máximo, e.g., 5 iterações), comparando as `introspections` da rodada anterior com a atual para atestar a estabilização matemática.
* **`evaluate_main_module`**: Antes do loop, executa uma passagem maciça do parser e avaliador (`typst_eval::eval`) para gerar a Content Tree e o escopo base.
* **`enforce_stabilization_and_delayed_errors`**: Lógica estritamente pura de promoção de erros. Avalia falhas de convergência usando `introspection::analyze` e verifica erros atrasados (`sink.delayed()`) para vetar a liberação do documento.
* **`deduplicate_diagnostics`**: Função de transformação pura (Set/Filtro). Recebe um vetor bruto de mensagens e metadados (`SourceDiagnostic`), calcula o hash individual de cada um, e retorna uma versão livre de duplicatas.
* **`generate_invalid_file_hints`**: Função puramente heurística de mapeamento. Recebe um erro (`FileError`) atrelado a um ID de arquivo ou caminho, e retorna sugestões heurísticas em string (ex: "verifique se a extensão deveria ser .typ ao invés de .pdf").
* **`validate_html_feature_flag`**: Lógica de roteamento puro que retorna as mensagens de aviso ("Warnings") ou falha dependendo se a flag booleana contextual para exportação HTML está habilitada.
* **`compile` e `trace`**: Controladores de fluxo que encapsulam a orquestração iterativa (`compile_impl`) interceptando ou silenciando diagnósticos/warnings. Na Tekt, viram casos de uso/orquestração (L2) que despacham as sub-etapas (Parser, Eval, Layout).

## 3. Efeitos Colaterais Identificados (Para os futuros Contratos L3)
O arquivo antigo embute efeitos colaterais pesados disfarçados atrás da interface polimórfica `&dyn World` e macros de telemetria.

* **I/O e File System Lookup**: Acesso imperativo aos arquivos fonte (`world.main()`, `world.source()`). Precisa ir ao mundo externo buscar os bytes e decodificá-los.
* **Acesso Ponto Flutuante / Tempo do Sistema (Telemetria)**: O módulo usa intensamente o `typst_macros::time` e `typst_timing::TimingScope` para logar o tempo de milissegundos dos processos. Isso é um IO forte (System Clock).
* **Descoberta de Dependências de Libs Globais**: O sistema obtêm as configurações carregadas do ambiente para usar bibliotecas padrão e fontes (`world.library()`). O acesso a configurações prévias do sistema injeta comportamento atado ao ambiente global.
* **Tracking de Cache de Compilação (Memoização por Memória)**: O uso intrusivo das diretrizes do `comemo::Constraint` e o `Protected` para gerir acessos de introspecção rastreados (quebra total de Purity) configura um forte vício em Side-Effect de Estado, já que os ponteiros mutáveis interagem com a concorrência global da máquina virtual. 

*(Criado contrato `00_nucleo/contracts/compiler_io.rs` para capturar a essência formal destas injeções de IO)*

## 4. Glossário / Assinaturas (Opcional)

* **`IterationHistory`**: Array interno (no legado, um `ArrayVec<D, MAX_ITERS>`) que armazena os outputs/introspecções das últimas rodadas para detecção estrita de estabilidade.
* **`Document` (Trait ou Type Class)**: Interface polimórfica crucial que o compilador devolve. Capaz de relatar Informação do Doc e Retornar seu Estado de Introspecção (`PagedDocument`, `HtmlDocument`). Implementações precisam de um método associado purificado `create()`.
* **`AsDocument` (Trait)**: Trait auxiliar para coerção de documento base para Object Trait.
* **`LibraryExt` (Trait)**: Extension trait fornecendo factories (default, builder) de Library injetadas com as rotinas padrão.
* **`ROUTINES`**: Tabela estática de ponteiros de função para carregar dependências circulares de forma dinâmica (substituto sujo para uma Injeção de Dependências ou Módulo Agregador limpo, que na Tekt L2/L3 deverá ser configurado claramente).
