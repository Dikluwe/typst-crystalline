# Auditoria de preparação para produção — branch Tekt — 2026-10-13

## Escopo e proveniência

Estudo solicitado pelo dono sobre partes provisórias que precisam ser corrigidas. Auditoria do workspace local **Tekt**, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. Não é uma auditoria de todos os caminhos nem certificação de produção. Nenhum consumer produtivo, L0, configuração ou binário foi alterado nesta auditoria.

O recibo `auditoria-producao-tekt-2026-10-13/before.json.gz` registra UTC inicial `2026-10-13T19:43:48.459397+00:00`, status completo, `git diff HEAD --stat` e hashes dos arquivos inspecionáveis de L1–L4, L0/núcleos e configurações. O diffstat inicial contém **490 arquivos alterados, 27625 inserções e 3218 remoções**; arquivos não rastreados constam separadamente no status. Não atribuir estes números a alterações deste estudo. `after.json.gz` e `verification.json` verificam preservação do estado.

Referência vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, árvore ratificada `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita em `00_nucleo/diagnosticos/typst-p1489-resync-vanilla.md`.

| Binário medido | SHA-256 |
|---|---|
| `lab/typst-original/target/release/typst` | `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` |
| `target/release/typst` | `fd758cf69301c297f6123f042ab19cc2de41e38a7f4af7fc351987728fa18655` |

As sondas PDF e HTML têm fonte, hash, argv, UTC, saída e diagnósticos em `probes.json.gz` e `html-probes.json.gz`, dentro da pasta do estudo. Os scripts e fixtures estão preservados. PDF foi avaliado por texto e caixas extraídos com Poppler; não houve comparação visual de raster nem prova de equivalência geral. HTML foi inspecionado como estrutura/texto serializado; não houve medição de CSS no navegador. Contagens abaixo pertencem exclusivamente a essas entradas e a esse estado.

## Medições antes da classificação

### 1. Conteúdo paginado

| Fixture | Tekt | Vanilla pinado | Observável |
|---|---|---|---|
| `header-static` | 2 páginas, HEADER aparece 2 vezes | igual | Controle estático passa |
| `header-context` | 2 páginas, nenhuma ocorrência de PAGE, sem diagnóstico | PAGE aparece 2 vezes | Cabeçalho contextual omitido |
| `footer-context` | 2 páginas, nenhuma ocorrência de PAGE, sem diagnóstico | PAGE aparece 2 vezes | Rodapé contextual omitido |
| `table-header` | 5 páginas, HEADER aparece 1 vez | 5 páginas, HEADER aparece 5 vezes | Falta repetição do grupo |
| `grid-header` | 2 páginas, HEADER aparece 1 vez | 2 páginas, HEADER aparece 2 vezes | Mesmo limite em grid |
| `table-no-header` | 4 páginas | 4 páginas | Controle sem grupo; não prova geometria geral |
| `footnote-overflow` | 1 página; extração termina em Note 12; warning de possível corte/sobreposição | 4 páginas; continuação até Note 49 | Continuação de nota longa ausente |
| `context-fragments` | fragmento `130:` separado de `1`, entre outros | `130:1` contínuo | Fronteiras de composição diferem |
| `callback-multilevel` | sucesso; leituras `(7,)` e `(10,)` | erro `missing argument: b` | Semântica/diagnóstico de callback ainda divergentes |

Na nota longa, a maior caixa extraída chega a yMax=221.33pt numa página de altura 220pt. Isto reforça a falha de recuperação do texto na página; não equivale a uma inspeção visual completa do PDF. A diferença de número de páginas, sozinha, não seria prova suficiente: a evidência relevante é a continuação ausente e o warning explícito.

Fontes e contratos que explicam o alcance atual:

- `01_core/src/compiler/layout/page_running.rs:73` compõe marginais; em `:86` e `:89` encaminha Content diretamente ao layout isolado. `03_infra/src/pipeline.rs:394` descobre contextos via travessia; `01_core/src/entities/content/traversal.rs:219` trata SetPage como terminal. **Hipótese causal:** contextos guardados nos marginais escapam à preparação por página. Refutar com teste isolado que mostre sua descoberta, execução e posicionamento efetivos. Não executar closures no layouter para contornar a hipótese.
- `00_nucleo/prompts/compiler/layout/page_running.md` especifica conteúdo explícito/realizado e não autoriza callbacks na unidade. Não promete paridade de largura intrínseca rica para numbering.
- `01_core/src/compiler/layout/grid/row_groups.rs:16` documenta emissão única dos grupos; `grid/cell_emission.rs:66` emite linhas e `grid/row_pagination.rs:13` decide quebras. O L0 `00_nucleo/prompts/compiler/layout/grid/row_groups.md` conserva grupos/spans multipágina como incompletos. O owner `00_nucleo/prompts/compiler/layout/grid/row_pagination.md` permite overflow quando uma linha excede uma página inteira.
- `00_nucleo/prompts/compiler/footnote_overflow_columns.md` é o owner de `01_core/src/compiler/layout/footnote_flush.rs`. O contrato atual **deliberadamente** força a primeira nota excessiva, com warning, para assegurar término; exclui a promessa de splitting/continuação vanilla. Este é um limite do contrato atual a completar, não prova de descumprimento desse contrato pelo consumer.
- Para callback Page, `00_nucleo/prompts/compiler/introspect/from_tags/counter_updates.md:61` distingue projeção da leitura e estado causal. A divergência medida exige caracterização da ordem/argumentos antes de decidir; não basta copiar um erro do oráculo nem normalizar tuples para fazer o teste passar.

### 2. HTML e bundle

Comandos executados no estado acima:

```sh
cargo test --offline --locked -p typst-wiring --test cli p117 -- --test-threads=1
cargo test --offline --locked -p typst-wiring --test cli p1485_bundle_introspeccao_atravessa_documentos -- --exact --test-threads=1
```

O primeiro filtro executou **9 testes: 3 passaram e 6 falharam**. O segundo executou **1 teste, que falhou**. Logs integrais: `html-tests.log` e `bundle-test.log`. Não extrapolar estes filtros para a suíte completa.

A reprodução bilateral com o release confirma a relevância das falhas no pin atual:

- P1171: Tekt emite `<div> A <br> B ... </div>`; vanilla remove espaços estruturais nas bordas/br.
- P1173/P1174/P1176/P1177: espaços estruturais extras entre blocos, após summary e entre anotações ruby. Os testes atuais usam strings exatas; a futura aceitação deve distinguir nós de texto/estrutura pertinentes de escolhas equivalentes de serialização. Este estudo não afirma que cada espaço extra altere pixels.
- P1178: título textual válido é rejeitado com `HTML raw text element cannot have non-text children`; vanilla aceita. Também falha o controle mínimo `#html.html[#html.head[#html.title[ABC]]#html.body[B]]`.
- Bundle com headings/labels e `query(heading).len()` em dois documentos: Tekt aborta com `HTML export does not yet support this content`; vanilla produz os dois HTMLs e mostra 3 em ambos. Controles de bundle com texto simples e heading simples passam nos dois binários. Não é falha universal de bundle.

Inspeção: `03_infra/src/export/html/whitespace.rs:31` só aplica a normalização ao Content::Sequence diretamente recebido; as classificações de vizinhos também reconhecem variantes diretamente. `03_infra/src/export/html/encoding.rs:12` e `:31` não atravessam Source/Located/Realized. `03_infra/src/export/html/inline.rs:34` já atravessa esses wrappers. `03_infra/src/pipeline/bundle.rs:308` faz outra projeção semântica parcial.

**Hipótese concreta, ainda não isolada por RED unitário:** transparência inconsistente dos carriers nas diferentes operações. A aceitação precisa testar o mesmo conteúdo com e sem wrappers e preservar estilo, identidade/efeitos e ordem. Não apagar todos os wrappers globalmente: cada operação possui obrigação própria. Owners atuais lidos:

- `00_nucleo/prompts/infra/export/html/whitespace.md`
- `00_nucleo/prompts/infra/export/html/encoding.md`
- `00_nucleo/prompts/infra/export/html/inline.md`
- `00_nucleo/prompts/infra/pipeline/html.md`
- `00_nucleo/prompts/infra/pipeline/bundle.md`
- `00_nucleo/prompts/wiring/tests/cli.md`

HTML e bundle permanecem experimentais também no vanilla medido; corrigir as regressões Tekt não certifica esses targets inteiros para produção.

### 3. CI, distribuição e travas

`cargo-metadata.json` confirma os pacotes root: typst-core, typst-shell, typst-infra, typst-wiring e typst-benches; sem features de pacote declaradas. Logo não basta renomear o pacote mantendo flags upstream inexistentes.

- `.github/workflows/release.yml:55` e `:59` compilam `typst-cli` com features upstream. `cargo build --offline -p typst-cli --release` falhou localmente por pacote inexistente (`release-command.log`). `Dockerfile:23` usa o mesmo pacote inexistente. O workflow/container completo não foi executado.
- `.cargo/config.toml:2` aponta `cargo testit` para `test-wrapper`, ausente do workspace; `cargo testit --help` falhou (`ci-testit.log`). CI usa esse alias em `.github/workflows/ci.yml:65`.
- CI também referencia `tests/fuzz` (`:123`, ausente na raiz) e `typst-library` (`:135`, ausente do workspace). Não contém etapa de crystalline-lint. Possui `-Dwarnings`; os testes locais emitem warnings, mas não foi executada a matriz remota com essas flags.

O comando `crystalline-lint .` terminou com **exit 0**, mas o resultado contém **541 warnings** e **1646 notas**, sem erros. `lint.sarif.json`, `lint-text.log` e `lint-summary.json` preservam a evidência, reproduzida em ambos os formatos. Distribuição dos warnings:

| Regra | Warnings |
|---|---:|
| V5 PromptDrift | 157 |
| V12 | 19 |
| V16 | 331 |
| V17 | 28 |
| V18 | 6 |

Dos V5, **151 estão em arquivos de produção sob src**, 1 em teste externo e 5 em snapshots históricos de diagnósticos. Exemplo: `01_core/src/compiler/eval/math.rs:1`. O snapshot inicial e o final são idênticos para esses arquivos; a deriva já estava na árvore auditada. V5 está configurado como warning em `crystalline.toml:63`; o threshold padrão de saída é error. **Exit 0 não significa zero violações nem linhagem íntegra.**

Os demais warnings são pistas que precisam de triagem, não defeitos funcionais automaticamente confirmados. Por exemplo, V12 sinaliza o carrier privado Published em `04_wiring/src/main.rs:22`, que é explicitamente autorizado por `00_nucleo/prompts/wiring.md`; vários outros V12 são fixtures de testes. Não mover tipos indiscriminadamente contra o L0 vigente.

### 4. Sinais de manutenção provisória

- `README.md:12` anuncia P990/5797 testes/zero violations; `:32` marca paridade de produção completa e `:38` lista contagens antigas. São declarações históricas, incompatíveis com seu uso como painel atual de prontidão.
- `01_core/src/entities/content.rs:840` ainda chama smallcaps de stub transparente; `01_core/src/compiler/layout/mod.rs:1472` implementa fallback com maiúsculas reduzidas. Não é ausência de implementação. A qualidade desse fallback é outra questão, não fechada pela fixture simples.
- `layout/mod.rs:1276` diz que Strong/Emph foram removidos, mas os braços atuais existem em `:1469`. Comentários de etapas passadas sobrevivem como se fossem estado vigente.
- `01_core/src/compiler/layout/grid/row_pagination.rs` possui teste vazio `modulo_compila_e_carrega`. A existência de `#[test]` não testa quebra, overflow ou grupos. Os L0 de paginação/emissão têm aceitação genérica de suíte/build/lint, insuficiente para delimitar o comportamento de produção.
- `00_nucleo/prompts/wiring/tests/cli.md` ainda indica comando com `-p typst`, pacote ausente. Revisar receitas operacionais junto com os testes.

## Decisão e ordem de correção

Classificação: os resultados de conteúdo, erro, contextos e continuação são linguagem/observáveis (ADR-0107). Não se exige copiar estruturas, algoritmo, igualdade Rust ou bytes PDF do vanilla. As hipóteses de causa estão explicitamente separadas das falhas reproduzidas. Os achados não justificam reescrever a arquitetura cristalina.

| Ordem | Pacote de trabalho | Critério concreto de fechamento |
|---|---|---|
| P0-A | Restaurar confiabilidade de linhagem e release | Auditar cada V5 de produção por owner; distinguir selo desatualizado de contrato divergente antes de resselo. CI compila o workspace real, roda testes e lint; release/container deixam de depender de pacotes/features upstream ausentes. Artefato inclui proveniência verificável. |
| P0-B | Cabeçalhos/rodapés contextuais | Fixtures contextuais mostram conteúdo e número correto em cada página; controle estático preservado. Testar estilos, total/final e efeitos sem execução duplicada. |
| P1-A | Regressões HTML/bundle | Sete testes focados voltam a passar; controles mínimo e com wrappers preservam DOM/texto, título/escape e consulta global. HTML continua target semântico separado. |
| P1-B | Paginação de grupos de tabela/grid | Header aparece nas páginas de continuação e respeita repeat; footer, linhas quebráveis/spans, bordas e ordem sem repetir efeitos/recipes. Não fechar apenas pela contagem de páginas. |
| P1-C | Continuação de notas e linhas maiores que a região | Conteúdo completo recuperável, sem sobreposição/corte silencioso; numeração/referência e progresso preservados. Contrato explícito para nota/linha que exige divisão. |
| P1-D | Fronteiras de parágrafo e callback Page | Isolar fragmentação textual e callbacks com fixtures mínimas; verificar texto/linhas, argumentos, ordem de efeitos e diagnósticos. Não esconder diferenças com normalização de saída. |
| P2 | Documentação e testes que expressem comportamento | Um índice atual de lacunas com fixture/owner/status/prova; remover comentários obsoletos dos módulos tocados e substituir smoke vazio por cenários causais. README deixa de funcionar como certificação baseada em corpus antigo. |
| Depois | Desempenho incremental | Só ampliar caches após dependências/efeitos/identidade comprovados e ganho medido no cenário real, além de correção. |

Primeiro lote de implementação recomendado: **triagem V5 + reparo das receitas CI/release**, seguido de **marginais contextuais e HTML/bundle**, que já possuem REDs pequenos e reproduzíveis. Separar a continuação multipágina em pacote próprio, porque os contratos atuais aceitam explicitamente o fallback limitado.

Cada mudança de produção começa pelo L0 proprietário lido e atualizado. Correções internas de paridade que preservem API e fase seguem fluxo contínuo ADR-0127, com resselo restrito e RED→GREEN. Se a solução acrescentar campo/método público, mover avaliação de fase, introduzir default/flag ou quebrar compatibilidade, redigir a proposta concreta e cumprir o gate humano previsto pela ADR. Este diagnóstico não é L0 e não autoriza arquitetura nova. Não há pedido de confirmação necessário para concluir este estudo.

Para o gate de lint: preservar V15/V26, separar evidência histórica do escopo de produção e tornar divergências relevantes bloqueantes depois da triagem. Não usar `--fix-hashes` global como prova de correção, não silenciar todos os warnings e não tratar detecção sintática como veredito sem ler o owner. Após cada pacote: testes comportamentais pertinentes, `cargo build` e `crystalline-lint .`; resultados e ressalvas separados, sem chamar exit 0 de zero violations.

O experimento de desempenho rejeitado em `00_nucleo/diagnosticos/page-observation-fold-2026-10-13.md` permanece rejeitado. Redução de alocações sem ganho no cenário real não deve preceder esses reparos de produção. Também não se propõe remover as guardas dos caches já medidos, trocar o enum fechado por despacho dinâmico, ou mover render para entidades em nome de atomização.

## Reprodução e limites finais

Da raiz do repositório:

```sh
python3 00_nucleo/diagnosticos/auditoria-producao-tekt-2026-10-13/audit.py probes
python3 00_nucleo/diagnosticos/auditoria-producao-tekt-2026-10-13/html-audit.py
crystalline-lint . --format sarif
```

Os scripts usam os binários indicados, registram seus hashes e sobrescrevem os recibos locais de probes: preserve o diretório antes de medir outro estado. Não recompilam o release automaticamente. Receitas, fontes e resultados originais permanecem nesta pasta; `evidence-manifest.json` registra seus hashes e ferramentas. A presente auditoria não rodou a suíte completa, builds multi-OS, container, PDF-UA ou um corpus geral de bibliografia/math/fontes. Isso delimita a conclusão, sem reduzir a prioridade dos defeitos já reproduzidos.
