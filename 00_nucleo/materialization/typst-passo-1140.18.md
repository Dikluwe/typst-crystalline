# Passo 1140.18 — Auditar e restaurar a função global `page`

**Estado:** executado — decisão γ, sem código L1  
**Data:** 2026-08-24  
**Continua:** P1140.17  
**Gate:** ADR-0127 obrigatório antes do código  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.18-page-global.md`

## 1. Objetivo

Resolver a divergência pública do identificador global `page` contra o vanilla
ratificado, sem confundir três superfícies diferentes:

1. `page(...)`, constructor que isola e compõe um corpo em uma ou mais páginas;
2. `#set page(...)`, regra que configura as páginas do fluxo corrente;
3. `location.page()`, método de introspecção.

O inventário P1140.17 classifica somente a primeira como `MISSING_BINDING`.
Este passo deve medir o contrato atual, reavaliar a decisão histórica P335 e,
se a restauração for confirmada, reutilizar a configuração e a paginação já
existentes em vez de criar uma segunda implementação de página.

## 2. Proveniência inicial

Estado que originou o passo:

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada;
- inventário: `00_nucleo/diagnosticos/superficie-linguagem-p1140.17.json`;
- catálogo vanilla: upstream/main ratificado `a51e02804`;
- inventário cristalino: build da working tree de P1140.17;
- medição final de P1140.17: `2026-08-24T13:31:30-03:00`;
- `git diff HEAD --stat` naquele momento:
  `83 files changed, 734 insertions(+), 505 deletions(-)`.

O inventário mede:

- vanilla: `page` presente, kind `function`, 19 parâmetros catalogados;
- cristalino: `page` ausente;
- classificação: `MISSING_BINDING`;
- fonte vanilla:
  `lab/typst-original/crates/typst-library/src/layout/page.rs:54`.

Esses 19 parâmetros são metadados observados, não autorização para reproduzir
defaults como constantes empíricas. Defaults devem vir de entidades e regras
semânticas donas ou ser medidos e especificados no L0.

## 3. Correção do enquadramento histórico

P335 removeu `native_page` porque o considerou uma forma-função “legacy”, com
zero call-sites internos, e declarou `#set page(...)` como caminho canónico.
Essa justificativa não basta para a superfície atual da linguagem:

- `PageElem` no vanilla ratificado conserva `#[elem(..., Construct)]`;
- a documentação atual diz explicitamente que, além de ser usada em set rules,
  a função pode renderizar seu argumento em páginas próprias;
- o constructor tem semântica diferente de uma set rule solta: cria fronteiras
  fracas, preserva página vazia com `FlushElem`, aplica estilos ao corpo e cria
  uma fronteira final;
- ausência de call-sites no Rust não prova ausência na linguagem, pois o
  consumidor é código Typst do utilizador.

Portanto, “legacy” é uma hipótese histórica refutada pela fonte ratificada e
pela superfície runtime. O passo não deve simplesmente ressuscitar o antigo
`native_page`: ele aceitava apenas `width`, `height` e `margin`, devolvia um
`SetPage` isolado e não implementava o contrato do constructor atual.

## 4. Medição obrigatória antes da decisão final

Executar no vanilla ratificado e no cristalino reconstruído, registrando stdout,
stderr, exit code, hora, HEAD e estatística da working tree:

### 4.1 Identidade e representação

```text
repr(type(page))
repr(page)
repr(page[alpha])
repr(page(body: [alpha]))
repr(std.page)
```

Confirmar se o body é aceito como argumento posicional por sintaxe de content
block, apesar de o catálogo runtime descrevê-lo como named. Não inferir essa
relação apenas dos metadados.

### 4.2 Fronteiras e morfologia

Medir documentos mínimos com:

1. conteúdo antes, dentro e depois de `page[...]`;
2. body vazio;
3. body que ocupa mais de uma página;
4. duas chamadas consecutivas;
5. chamada dentro de bloco/container;
6. `page(width: ..., height: ...)[...]` versus
   `#set page(width: ..., height: ...)` no nível superior;
7. propriedades anteriores retomadas depois da chamada;
8. `show page` para confirmar que o constructor não torna páginas
   selecionáveis por show rule.

Aceitação deve ser por semântica, sintaxe e morfologia. Não exigir árvore Rust,
sequência interna idêntica nem bytes PDF.

### 4.3 Parâmetros

Medir, sem tentar implementar todos de uma vez:

- `paper`, `width`, `height`, `flipped`, `margin`;
- `bleed`, `binding`, `columns`, `fill`;
- `numbering`, `supplement`, `number-align`;
- `header`, `header-ascent`, `footer`, `footer-descent`;
- `background`, `foreground`, `body`;
- argumento nomeado desconhecido e posicionais excedentes.

Para cada parâmetro, classificar:

- **A — já representado e consumido** pelo caminho `#set page` cristalino;
- **B — parcialmente representado**, mas com perda semântica conhecida;
- **C — ausente** de entidade, eval ou layout;
- **D — apenas metadado/mecânica**, sem efeito observável próprio.

A classificação deve citar `file:line`, indicar inferências e dizer qual
medição as refutaria.

## 5. Auditoria L0

Ler integralmente, antes de propor arquitetura:

1. `00_nucleo/prompts/compiler/stdlib/layout.md`;
2. `00_nucleo/prompts/compiler/eval.md`;
3. `00_nucleo/prompts/entities/content.md`;
4. `00_nucleo/prompts/compiler/layout.md`;
5. prompts dos consumers realmente tocados pela implementação;
6. ADR-0107, ADR-0108, ADR-0109 e ADR-0127.

O L0 `compiler/stdlib/layout.md` atualmente proíbe `native_page` com base na
decisão P335. Como a fonte ratificada contradiz a premissa “legacy”, restaurar o
binding exige atualizar explicitamente esse L0; não basta acrescentar código.

## 6. Decisão arquitetural a produzir

Depois da medição, escrever no diagnóstico uma destas decisões:

### α — restauração completa agora

Escolher somente se todos os parâmetros relevantes já têm representação e
consumers suficientes para reproduzir o constructor no nível da linguagem.

### β — restauração nucleada por subconjuntos

Escolher se a fronteira do constructor pode ser implementada corretamente com
um subconjunto já suportado. Nesse caso, o L0 inicial deve declarar que está
incompleto e nomear os passos que completarão cada grupo de parâmetros, conforme
a regra de divisão de constructor entre passos. Parâmetro reconhecido mas
silenciosamente ignorado é proibido.

### γ — adiar o binding e corrigir pré-requisitos

Escolher se expor `page` agora criaria uma função publicamente enganosa ou
exigiria duplicar a paginação. O diagnóstico deve então ordenar os
pré-requisitos em passos independentes e manter `page` como lacuna conhecida.

A opção preferida só pode aparecer depois da tabela A/B/C/D e das medições.

## 7. Fronteira de implementação candidata

Se α ou β forem sustentadas:

- o constructor pertence a uma unidade atomizada de stdlib de layout, não a
  `eval/mod.rs` nem à entidade `Content`;
- `make_stdlib` apenas registra/reexporta o binding;
- parsing e validação de argumentos devem reutilizar helpers existentes do
  caminho `#set page` quando isso não inverter dependências;
- a saída deve representar as fronteiras semânticas do constructor: quebra
  fraca inicial, marcador não vazio/invisível equivalente ao `FlushElem`, body
  sob configuração local e quebra de fronteira final;
- se o cristalino não possuir equivalentes dessas unidades, especificá-las no
  L0 antes do código; não aproximar com um único `Content::SetPage`;
- o constructor não pode vazar sua configuração para o conteúdo posterior;
- atomização segue ADR-0109: lógica de layout permanece nos arquivos donos e o
  despacho estático/exaustivo permanece.

O nome exato do arquivo só será decidido após verificar a atomização vigente de
`compiler/stdlib/layout`; não criar `page.rs` apenas por simetria nominal.

## 8. Gate ADR-0127

Restaurar `page` altera contrato público e comportamento por defeito da chamada.
O fluxo obrigatório é:

1. concluir a medição e a decisão α/β/γ;
2. atualizar todos os L0s afetados;
3. guardar os L0s e normalizar seus hashes;
4. registrar os hashes no passo;
5. **PARAR**;
6. aguardar confirmação explícita do dono;
7. somente então escrever testes RED e código.

Se a decisão for γ, não há código público neste passo e não se fabrica um gate
para uma implementação ainda não especificada.

## 9. RED→GREEN após o gate

Os testes mínimos, ajustados ao resultado medido, devem cobrir:

1. `type(page) == function` global e por `std.page`;
2. forma posicional/named do body ratificada;
3. isolamento do body entre conteúdo anterior e posterior;
4. body vazio preserva uma página;
5. body multipágina permanece dentro das fronteiras;
6. propriedades locais não vazam após a chamada;
7. parâmetros do subconjunto autorizado têm efeito observável;
8. parâmetros ainda não autorizados produzem erro explícito, se β;
9. argumento desconhecido e posicionais excedentes reproduzem diagnósticos;
10. `#set page(...)` e `location.page()` não regridem;
11. `show page` mantém o comportamento vigente.

Registrar o RED real. Um teste que passa antes do código não prova a lacuna e
deve ser reformulado ou classificado como controle.

## 10. Rebaseline e validação

Depois do GREEN:

1. reconstruir `target/release/typst`;
2. gerar inventário `superficie-linguagem-p1140.18.json`;
3. gerar probes `superficie-linguagem-p1140.18-probes.json`;
4. confirmar que `page` sai de `MISSING_BINDING` somente se o binding foi
   efetivamente restaurado;
5. executar testes focados e `cargo test -p typst-core --lib`;
6. executar `cargo build --workspace`;
7. executar `crystalline-lint .` com exit 0;
8. executar `git diff --check`;
9. registrar HEAD, hora e `git diff HEAD --stat` de cada número usado para
   fechar o passo.

Uma reclassificação para `UNVERIFIED_METADATA` é aceitável se presença e kind
forem equivalentes, mas o instrumento cristalino continuar sem metadados de
assinatura. Isso não substitui os testes de parâmetros.

## 11. Aceitação

O passo fecha quando:

1. a decisão P335 foi reavaliada contra a fonte ratificada, sem preservar a
   premissa refutada de “legacy”;
2. as 19 entradas do catálogo foram classificadas A/B/C/D com fonte;
3. α, β ou γ foi escolhida depois da medição;
4. os L0s foram atualizados antes de qualquer código;
5. para α/β, o gate foi confirmado e o RED→GREEN registrado;
6. nenhuma propriedade é aceita e descartada silenciosamente;
7. o constructor, se exposto, isola o body e não vaza configuração;
8. controles de `#set page`, paginação, colunas, numeração e introspecção passam;
9. inventário, probes, testes, build e linter têm proveniência reproduzível;
10. o diagnóstico final distingue lacunas fechadas, parciais e adiadas.

## 12. Fora de escopo

- implementar `path`;
- mudar `location.page()`;
- tornar `show page` funcional;
- redesenhar toda a paginação ou a style chain;
- copiar a mecânica interna do vanilla como requisito de igualdade;
- introduzir defaults numéricos medidos diretamente no código;
- aceitar parâmetros sem consumer semântico;
- igualdade byte a byte de PDF.

## 13. Resultado da execução

A auditoria escolheu **γ — adiar o binding e corrigir pré-requisitos**.

Medições determinantes:

- `repr(type(page))` é `"function"` no vanilla e `unknown variable page` no
  cristalino;
- `page[alpha]` produz morfologicamente `pagebreak(weak: true)`, `flush()`,
  body e outra fronteira fraca;
- `page(body: [alpha])` é erro porque `body` é posicional, corrigindo a leitura
  insuficiente do catálogo runtime;
- `page[]` conserva `flush()` mesmo com body vazio;
- duas chamadas consecutivas preservam duas sequências delimitadas;
- `page` dentro de `block` mantém as fronteiras no body;
- a fonte vanilla aplica um mapa de estilos somente à sequência delimitada.

Classificação final dos 19 parâmetros:

- A: 3 (`width`, `height`, `columns`);
- B: 3 (`margin`, `numbering`, `body`);
- C: 13 (`paper`, `flipped`, `bleed`, `binding`, `fill`, `supplement`,
  `number-align`, `header`, `header-ascent`, `footer`, `footer-descent`,
  `background`, `foreground`);
- D: 0.

O L0 `compiler/stdlib/layout.md` foi corrigido para retirar a premissa
“legacy”, registrar a lacuna e dividir explicitamente os pré-requisitos entre
P1140.19, P1140.20 e P1140.21. SHA-256 final do L0:
`573b7c0642299ba3115c570b73a99a75c0a22d77c887000e70f51ccd1116fe9d`.
O arquivo L1 vinculado usa o formato antigo de linhagem sem `@prompt-hash`; o
linter não reportou drift nem ofereceu correção. Como γ não expõe binding nem
altera código ou comportamento, não há gate de implementação neste passo.

Proveniência da decisão: HEAD
`45b547073d7686cdd5d3e3030c82de3e22ec395f`, working tree não commitada,
medição em `2026-08-24T13:36:50-03:00`, com
`83 files changed, 734 insertions(+), 505 deletions(-)` antes das alterações
documentais deste resultado.

Validação documental final em `2026-08-24T13:38:13-03:00`:
`crystalline-lint --fix-hashes .` respondeu `Nothing to fix`,
`crystalline-lint .` terminou com exit 0 e `git diff --check` passou. A árvore
então tinha `84 files changed, 775 insertions(+), 508 deletions(-)`.
