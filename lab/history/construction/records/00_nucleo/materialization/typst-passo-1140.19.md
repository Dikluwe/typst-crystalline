# Passo 1140.19 — Fronteira de page-run e restauração de `PageConfig`

**Estado:** executado  
**Data:** 2026-08-24  
**Continua:** P1140.18  
**Prepara:** P1140.20 e P1140.21  
**Gate:** ADR-0127 obrigatório antes do código  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.19-page-run.md`

## 1. Objetivo

Materializar o mecanismo interno mínimo que permitirá ao futuro constructor
`page(...)` isolar um body em um page-run e restaurar a `PageConfig` anterior
depois dele, sem expor ainda o binding global `page` e sem implementar as 13
propriedades ausentes classificadas por P1140.18.

O mecanismo deve representar três observáveis ratificados:

1. o body começa numa fronteira de página;
2. um body vazio conserva uma página;
3. ao terminar o body, dimensões, margens, colunas e numeração anteriores voltam
   a reger o conteúdo seguinte.

Este passo não autoriza aproximar o constructor com `SetPage + body` nem
introduzir uma pilha global mutável.

## 2. Proveniência inicial

Base medida por P1140.18:

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada;
- vanilla ratificado: upstream/main `a51e02804`;
- diagnóstico:
  `00_nucleo/diagnosticos/typst-p1140.18-page-global.md`;
- L0 corrigido:
  `00_nucleo/prompts/compiler/stdlib/layout.md`, SHA-256
  `573b7c0642299ba3115c570b73a99a75c0a22d77c887000e70f51ccd1116fe9d`.

P1140.18 mediu `page[alpha]` como sequência pública formada por
`pagebreak(weak: true)`, `flush()`, body e outra fronteira fraca. A fonte
vanilla em `layout/page.rs:504-523` esclarece que a fronteira final é boundary,
mais fraca que `weak`, e que `flush` mantém a página mesmo quando o body está
vazio.

## 3. Estado cristalino a medir novamente

Antes de decidir tipos ou variantes, confirmar com `file:line`:

- `Content::Pagebreak` possui `weak`, presença explícita e `to`, mas não
  distingue boundary interna;
- `Content::SetPage` transporta somente `width`, `height`, `margin`,
  `numbering` e `columns`;
- `set_page::layout` clona a configuração atual para calcular a nova, mas
  substitui `layouter.page_config` sem empilhar/restaurar;
- `Content::Empty` pode ser eliminado e não serve como marker que conserva
  página;
- não existe equivalente de `FlushElem` na entidade;
- `Content::Styled` transporta estilos textuais/semânticos, não deve ser usado
  para esconder mutação de `PageConfig`;
- `new_page()` e `finish()` possuem flushes pendentes cuja ordem não pode ser
  duplicada por um novo consumer.

Toda inferência deve registrar o teste que a refutaria.

## 4. Medição diferencial obrigatória

Medir no vanilla e no cristalino, com HEAD, hora e estatística da árvore:

1. conteúdo `before + page[...] + after` com dimensões distintas;
2. body vazio no início, meio e fim do documento;
3. body que ocupa duas ou mais páginas;
4. dois page-runs consecutivos com configurações diferentes;
5. page-run contendo `pagebreak()` explícito;
6. page-run com `height: auto` e `width: auto`;
7. page-run em `block`, `columns` e outro container que proíba pagebreak;
8. `#set page(...)` antes, dentro e depois do run;
9. numeração e colunas antes/depois, limitadas ao suporte atual;
10. páginas vazias consecutivas, para distinguir weak de boundary.

Registrar semântica, sintaxe e morfologia. Contagem de páginas e dimensões só
podem fechar decisão com proveniência completa. Bytes PDF e igualdade da árvore
Rust não são critérios.

## 5. Auditoria L0

Ler integralmente e validar antes de escrever código:

1. `00_nucleo/prompts/entities/content.md`;
2. `00_nucleo/prompts/entities/layout_types.md`;
3. `00_nucleo/prompts/compiler/layout.md`;
4. L0 proprietário de `compiler/layout/pagebreak.rs`;
5. L0 proprietário de `compiler/layout/set_page.rs`;
6. `00_nucleo/prompts/compiler/stdlib/layout.md`;
7. ADR-0107, ADR-0108, ADR-0109 e ADR-0127.

Se os arquivos `pagebreak.rs` ou `set_page.rs` estiverem ligados a prompts
históricos que não descrevem este contrato, atualizar os L0s verdadeiramente
donos; não acrescentar o mecanismo a um prompt por conveniência.

## 6. Decisão de representação

Depois da medição, comparar pelo menos:

### α — `Content::PageRun`

Elemento interno com configuração parcial e body. O consumer de layout salva a
configuração vigente, inicia a fronteira, aplica a configuração local, compõe o
body, fecha o run e restaura o snapshot.

Vantagem: posse e lifetime explícitos. Risco: nova variante pública de enum e
propagação pelos matches exaustivos.

### β — eventos tipados `PageRunStart`/`PageRunEnd` + marker

Eventos numa sequência, com identidade de run ou snapshot transportado.

Vantagem: morfologia próxima da sequência vanilla. Risco: sequência malformada,
restauração em caminhos aninhados e estado aberto atravessando consumers.

### γ — extensão de `SetPage` com operação push/pop

Adicionar modo interno ao marker existente.

Vantagem: reutiliza o consumer. Risco: mistura set-rule pública progressiva com
escopo lexical e torna fácil aceitar pares desequilibrados.

A recomendação inicial é **α**, mas é inferência, não decisão: a medição e a
auditoria dos matches podem refutá-la se um `Content::PageRun` não puder
preservar corretamente as fronteiras dentro do fluxo de layout. A decisão final
deve aparecer somente depois das evidências.

Não escolher por menor diff. Escolher pela capacidade de impedir vazamento de
configuração por construção.

## 7. Contrato mínimo candidato

Se α for confirmada, o L0 deve especificar semanticamente, sem copiar a
mecânica vanilla:

- `PageRun` é contentor interno e não selecionável por show rule;
- contém somente os cinco campos já representados por `SetPage` mais `body`;
- campos `None` preservam a configuração vigente ao entrar no run;
- a configuração efetiva é snapshot local, não style global;
- entrar fecha a página corrente conforme a política weak ratificada;
- body vazio produz uma página material;
- sair fecha o último trecho do run e restaura o snapshot anterior antes do
  próximo conteúdo;
- runs aninhados restauram em LIFO por descendência normal de chamadas, sem
  estado global;
- erro/retorno antecipado não deixa configuração alterada;
- `plain_text` devolve somente o body;
- `is_empty` deve refletir que o run tem efeito mesmo com body vazio;
- mapeamentos de conteúdo recursam no body;
- `repr` interno só será exposto quando P1140.21 medir a forma pública.

O marker equivalente a `flush()` deve ser representado somente se a medição
mostrar que `PageRun` sozinho não consegue conservar a página vazia. Não criar
uma variante separada apenas para imitar a árvore vanilla.

## 8. Atomização

Se houver `Content::PageRun`, sua entidade dona deve ficar em
`entities/elements/page_run.rs`. A lógica de composição fica em
`compiler/layout/page_run.rs`, chamada por braço magro no match exaustivo,
seguindo ADR-0109 forma B.

Parsing e binding de `page` continuam fora de escopo. `set_page.rs` pode expor
um helper interno para aplicar configuração se a auditoria demonstrar que ele
não mistura progressão de set-rule com escopo de run. Não criar dependência
`entities → compiler`, `dyn`, vtable ou mapa de propriedades genérico.

## 9. Gate ADR-0127

A introdução de `PageRun` altera o contrato público do enum `Content` e muda
uma fase de layout. Portanto:

1. concluir medições e escolher α/β/γ;
2. escrever os L0s afetados;
3. normalizar e registrar hashes;
4. **PARAR**;
5. aguardar confirmação explícita do dono;
6. só depois escrever testes RED ou código L1.

Mesmo sem binding global, este gate é obrigatório. Em caso de decisão que não
altere `Content`, reclassificar formalmente antes de dispensá-lo; a dúvida
mantém a parada.

## 10. RED→GREEN após confirmação

Testes mínimos:

1. RED demonstra que `SetPage + body` vaza configuração para o conteúdo
   seguinte;
2. run com largura/altura locais restaura dimensões anteriores;
3. run com margens locais restaura os quatro lados anteriores;
4. run com colunas e numeração restaura ambas;
5. body vazio produz uma página;
6. body multipágina fecha apenas seu próprio conjunto;
7. dois runs consecutivos não compartilham configuração;
8. runs aninhados restauram em ordem LIFO;
9. pagebreak explícito dentro do body não encerra o run prematuramente;
10. `height:auto` e `width:auto` permanecem locais;
11. `#set page(...)` fora de runs mantém o comportamento progressivo atual;
12. todos os matches de `Content` permanecem exaustivos e semanticamente
    classificados.

Os testes devem usar observáveis de página, dimensões e conteúdo, não comparar
uma sequência interna exata com o vanilla.

## 11. Validação

Após GREEN:

1. testes focados;
2. `cargo test -p typst-core --lib`;
3. `cargo build --workspace`;
4. `crystalline-lint --fix-hashes .` apenas após os L0s;
5. `crystalline-lint .` com exit 0;
6. `git diff --check`;
7. probes de `page` continuam falhando como `MISSING_BINDING`, pois P1140.19
   não expõe a função;
8. registrar HEAD, hora e `git diff HEAD --stat` de todos os números usados.

Não gerar um rebaseline que finja fechar a lacuna pública.

## 12. Aceitação

O passo fecha quando:

1. weak, boundary e preservação de página vazia foram distinguidos por medição;
2. α/β/γ foi decidida após a auditoria;
3. o L0 especifica restauração sem vazamento e runs aninhados;
4. o gate ADR-0127 foi confirmado;
5. RED prova o vazamento do mecanismo antigo;
6. GREEN prova isolamento de todos os cinco campos já suportados;
7. body vazio e multipágina têm comportamento ratificado;
8. nenhuma propriedade de P1140.20 foi implementada por aproximação;
9. `page` permanece ausente da stdlib;
10. testes, build, linter e proveniência passam.

## 13. Fora de escopo

- binding global ou `std.page`;
- parsing de `page(...)`;
- `paper`, `flipped`, `bleed`, `binding`, `fill`, headers, footers, background
  ou foreground;
- completar numbering funcional ou supplement;
- tornar `show page` efetivo;
- mudar `location.page()`;
- igualdade da estrutura Rust ou bytes PDF;
- defaults empíricos inseridos no código.

## 14. Resultado pré-gate

A medição confirmou a opção **α — `Content::PageRun`**. Alternativas β e γ
foram rejeitadas porque tornam possível estado de run desequilibrado e misturam
escopo lexical com `SetPage` progressivo.

Observáveis medidos:

- isolamento simples: `200×200 → 100×120 → 200×200`, 3 páginas;
- body vazio no meio: mesmas 3 páginas, com página central sem texto;
- body vazio sozinho: 1 página `100×120`;
- consecutivos: 2 páginas, `100×120 → 140×160`;
- aninhado: `180×180 → 100×120 → 180×180 → 240×240`;
- pagebreak interno: duas páginas `100×120`, depois `200×200`;
- configuração dentro de block/columns: erro
  `page configuration is not allowed inside of containers`.

L0s preparados antes do código:

1. novo `entities/elements/page_run.md`;
2. `entities/content.md`;
3. `compiler/layout.md`;
4. `compiler/atomizacao_elementos.md`.

Nenhum teste nem lógica L1 foi alterado. Os hashes foram normalizados pelo
linter, atualizando somente headers de linhagem:

- `entities/content.md` → `a3f5a55b`;
- `compiler/layout.md` → `7962f4b2`;
- `compiler/atomizacao_elementos.md` → `6a13ed42` nos módulos atomizados;
- novo `entities/elements/page_run.md` → SHA-256
  `51dd2d2253a1094fb6e3c024c9ceababf89530fe9c8fa74556b071386085473e`;
  ainda sem arquivo L1 vinculado.

Validação pré-gate em `2026-08-24T13:44:51-03:00`:

- HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada;
- `84 files changed, 833 insertions(+), 508 deletions(-)`;
- `crystalline-lint .`: exit 0;
- `git diff --check`: aprovado.

**PARADA ADR-0127:** aguardar confirmação explícita do dono antes de escrever
testes RED ou implementar `PageRunElem` e seu consumer de layout.

## 15. Resultado pós-confirmação

O dono confirmou o gate em 2026-08-24. O RED compilável falhou com
`no variant or associated item named page_run found for enum Content`. Depois
da implementação, 8 testes P1140.19 passaram.

Materializado:

- `PageRunElem` atomizado em `entities/elements/page_run.rs`;
- `Content::PageRun` e constructor interno tipado;
- consumer forma B em `compiler/layout/page_run.rs`;
- snapshot/restauração local de `PageConfig`;
- boundary final que conserva body vazio, mas suprime página-cauda vazia;
- erro de configuração de página em sub-frame/container;
- recursão por introspecção, transformação de conteúdo e queries L3;
- nenhum binding `page` ou `std.page`.

Validação final:

- 8/8 testes focados;
- `cargo test -p typst-core --lib`: 5.170 aprovados, zero falhas;
- `cargo build --workspace`: aprovado;
- `crystalline-lint .`: exit 0;
- `git diff --check`: aprovado;
- probe `repr(type(page))`: continua `unknown variable page`, como exigido.

Hashes finais de linhagem: `page_run` entidade `5db04263`, `content`
`a3f5a55b`, layout/page-run `7962f4b2`, query helpers `3e7c7067`.

Proveniência final: HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`,
working tree não commitada, `2026-08-24T13:57:11-03:00`,
`90 files changed, 1027 insertions(+), 511 deletions(-)`.
