# Auditoria de atomização por comportamento e melhorias possíveis no L1

## Síntese

Há espaço para atomizar mais o L1 sem mudar o modelo de `Content`, o `match` exaustivo ou a direção dos imports. A primeira extração recomendada é a **realização textual de `compiler/eval/rules.rs`**: ela já forma uma unidade de comportamento com estado, entrada e helpers próprios. Depois vêm os braços `Heading` e `Label` do walk de introspecção, que ainda concentram lógica no hub apesar de existirem módulos filhos. As nativas matemáticas estruturais também são divisíveis, mas têm superfície e L0 mais extensos.

Atomização é uma melhoria de localidade e manutenção segundo [ADR-0109](../adr/typst-adr-0109-atomizacao.md). Não é uma otimização de tempo por si. As hipóteses de desempenho deste documento são separadas das extrações estruturais e exigem medição antes de alterar algoritmos.

## Base da inspeção

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado exato (`git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`) e SHA-256 dos arquivos L0, ADR e consumers usados nesta análise foram capturados em **2026-09-30 14:43:16.377091 UTC** em [state.json.gz](l1-atomizacao-2026-09-30/state.json.gz). Outros trabalhos já alteravam 198 caminhos do L1; não foram sobrescritos. As linhas indicadas abaixo referem-se a esse snapshot e podem deslocar-se.

Inspecionei os owners L0 vigentes antes de classificar opções. A cardinalidade [ADR-0129](../adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md) exige um Prompt L0 exclusivo por consumer produtivo: um novo arquivo de produção requer Prompt L0 próprio; não basta mover o header. O L0 do hub deve ceder a regra que passa ao filho e manter a interface de delegação. O fluxo de correção interna da ADR-0127 permite L0 primeiro e resselo contínuo, sem gate humano, desde que contrato público, comportamento padrão, fase e compatibilidade permaneçam iguais.

## Candidatos avaliados

| Prioridade | Local | Evidência de responsabilidades | Decisão |
| --- | --- | --- | --- |
| 1 | [`eval/rules.rs`](../../01_core/src/compiler/eval/rules.rs) | `eval_set_rule` começa em :1629 e ocupa múltiplos schemas; `eval_show_rule` começa em :3568; realização textual e `TextualSnapshot` concentram-se em :3930–4366. O owner L0 já distingue essas semânticas. | Extrair primeiro realização textual para `eval/rules/textual.rs`, com owner `prompts/compiler/eval/rules/textual.md`. Manter no hub o roteamento `set`/`show` e reexports privados necessários. Depois estudar schemas de `set` por família. |
| 2 | [`introspect.rs`](../../01_core/src/compiler/introspect.rs) | `walk` começa em :1897; braços `Heading` e `Label` ainda contêm captura, recursão e tags em :2080–2290 e :2320–2390. Já existem [`heading.rs`](../../01_core/src/compiler/introspect/heading.rs) e [`labelled.rs`](../../01_core/src/compiler/introspect/labelled.rs) com cálculo da mesma unidade. | Mover o comportamento completo desses dois braços para free functions nos filhos existentes, deixando apenas delegação no `match`. Ajustar os três L0 existentes, preservando Location, ordem Start/End e passagem causal da chain. |
| 3 | [`stdlib/structural/math.rs`](../../01_core/src/compiler/stdlib/structural/math.rs) | Equação/frac/accent, vetores/matrizes/cases, delimitadores/over-under e montagem do módulo ocupam blocos distintos antes dos testes. O arquivo também abriga vários testes inline. | Dividir construtores por família sem mover layout para entities. Cada arquivo novo precisa de L0 próprio; `make_math_module` fica como catálogo estático magro. Escolher uma família por passo para preservar spans, casts e nomes nativos. |
| 4 | [`eval/bindings/field_access.rs`](../../01_core/src/compiler/eval/bindings/field_access.rs) | Roteamento de campo em :557–1056 e projeção/métodos de `Content` em :1192–1589 são comportamentos distintos; depois começam blocos de teste. | Extrair projeção de `Content` para filho com owner próprio, mantendo o dispatch de `Value` exaustivo no hub e sem tornar métodos isolados first-class. |
| Baixa | [`stdlib/calc.rs`](../../01_core/src/compiler/stdlib/calc.rs) | O código produtivo termina perto de :1570; grande parte das 4512 linhas são testes inline iniciados em :1573. | Mover testes por família quando houver intervenção nesse módulo. O tamanho bruto não justifica quebrar a implementação antes dos candidatos acima. |
| Preservar | [`layout/mod.rs`](../../01_core/src/compiler/layout/mod.rs) | O hub já delega render a numerosos filhos e o [L0 de layout](../prompts/compiler/layout.md) atribui a ele estado, transação, despacho exaustivo e composição terminal. | Não fragmentar por contagem de linhas. Uma extração só é válida quando um comportamento ainda específico permanecer no hub e o owner for relido. |

Os números de linha e de tamanho são inventário estrutural, não prova de custo de CPU. Os arquivos de testes embutidos explicam parte importante dos tamanhos de `introspect.rs`, `calc.rs` e `field_access.rs`; portanto linhas totais não são uma boa ordenação sozinha.

## Primeiro recorte concreto

O bloco textual de `rules.rs` contém `TextualSnapshot`, `intercept_text_element`, `realize_textual_parts`, agrupamento e splice de matches, projeção das faixas originais e normalização de espaços. Ele tem um motivo único para mudar: a semântica de aplicação de show literal/regex sobre texto realizado. A [spec vigente de rules](../prompts/compiler/eval/rules.md) já especifica esse comportamento separadamente dos schemas `set` e do `show` por elemento.

Recorte proposto:

1. Medir o estado pré-movimento com testes focais de literal, regex, espaço inferido/explicito, CJK, snapshot lexical, revogação/guard e erros de span já existentes. Não criar testes que apenas verifiquem a nova localização do código.
2. Criar o Prompt L0 `compiler/eval/rules/textual.md` como owner exclusivo do novo consumer; no L0 pai, manter apenas a fronteira e as invariantes compartilhadas pinadas por Núcleo quando forem realmente compartilhadas.
3. Mover estado e funções do comportamento textual para `01_core/src/compiler/eval/rules/textual.rs`. O `match` e o roteamento nativos continuam estáticos; helpers do novo filho podem receber referências ao contexto causal sem ampliar API pública.
4. Repetir os mesmos testes, `cargo build` e `crystalline-lint .`; validar V15/V26 antes de `--fix-hashes`. Comparar o diff para garantir que a mudança é só estrutural e não altera ordem de avaliação, normalização ou mensagens.

Esse recorte melhora leitura independente e reduz a concentração do arquivo sem apostar em ganho de CPU. A extração dos schemas `set` é posterior porque a função mistura políticas de cast, warnings e efeitos na `StyleChain`; movê-la inteira de uma vez cria um diff mais difícil de verificar.

## Melhorias de desempenho a medir dentro do L1

1. **Grid:** [grid.rs:382](../../01_core/src/compiler/layout/grid.rs) mede células para altura e [grid.rs:776](../../01_core/src/compiler/layout/grid.rs) chama `layout_sub_frame` novamente para emissão. É trabalho repetido comprovado no caminho de grid, ainda sem parcela end-to-end medida. Antes de cache, medir chamadas por célula e preservar `inset`, alinhamento, posição, páginas e comportamento de `TableCell`/`GridCell`.
2. **Eval e stdlib:** [eval/mod.rs:1219](../../01_core/src/compiler/eval/mod.rs) executa duas passagens independentes; [eval/mod.rs:1259](../../01_core/src/compiler/eval/mod.rs) monta a stdlib dentro da closure de cada passagem. O [L0 de eval](../prompts/compiler/eval.md) exige as duas visões causais e scopes frescos, mas permite investigar compartilhamento de uma base imutável se sua identidade, sombras e `std` permanecerem iguais. Contagem de montagens e tempo exclusivo vêm antes da mudança.
3. **Show textual:** [`TextualSnapshot::same_group`](../../01_core/src/compiler/eval/rules.rs) compara `styles.collapse()` para snapshots adjacentes; `intercept_text_element` filtra e clona regras de nó. Perfilar número de chamadas, comprimento das chains, alocações e presença real de regras nos mesmos documentos. Um cache ou fast path só é justificável com saída de linguagem idêntica e queda em tempo de fase compatível.
4. **Fixpoint paginado:** [layout/mod.rs:2600](../../01_core/src/compiler/layout/mod.rs) cria candidato novo até convergir; normalmente pode sair na primeira iteração, portanto o limite cinco não é custo fixo. Medir passes efetivos em TOC, referências, callbacks e documentos simples antes de propor reutilização. O L0 vigente proíbe assumir o antigo atalho “sem outline = uma tentativa”.

A [comparação atual com vanilla](auditoria-l1-vanilla-performance-2026-09-30.md) localiza a maior diferença de CLI no primeiro catálogo de fontes, predominantemente L3. Assim, nenhuma das propostas L1 acima deve ser apresentada como correção dos aproximadamente 56–88 ms de diferença nos documentos com texto sem perfil que separe fontes, métricas, eval e layout. O objetivo deste refinamento é tornar o L1 mais legível e preparar experimentos de desempenho com hipóteses específicas.

## Critério de conclusão da primeira extração

O resultado é aceitável se o L0 pai/filho e headers mantiverem ownership 1:1, `crystalline-lint .` continuar sem violações, o `match` nativo permanecer estático e exaustivo, os testes focais permanecerem verdes, e o diff não alterar observáveis de linguagem ou diagnósticos. Não se declara ganho de desempenho sem medição bilateral ou controle A/B sobre o mesmo input e binários identificados.
