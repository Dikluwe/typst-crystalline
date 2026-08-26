# L0 — Supplement e referências de página
Hash do Código: 0d79ef0e

## Linhagem e alcance

Este prompt legitima `page.supplement`, seu snapshot/sealing por página e a
forma `ref(form: "page")`. Não legitima ainda a exposição do constructor
público `page`/`std.page`.

Fonte de paridade: vanilla ratificado `a51e02804`, especialmente:

- `typst-library/src/layout/page.rs:315-329`;
- `typst-layout/src/pages/run.rs:145-149,233-235`;
- `typst-layout/src/document.rs:98-103`;
- `typst-layout/src/introspect.rs:29-56,122-129`;
- `typst-library/src/model/reference.rs:135-257,334-355`;
- `typst-library/translations/*.txt`, chave `page`.

## Domínio de supplement

`PageSupplement` preserva três estados de valor:

- `Auto`: resolve para o nome localizado de página no idioma vigente;
- `None`: resolve para conteúdo vazio;
- `Content(Content)`: preserva o conteúdo explícito, inclusive vazio.

O delta de set-rule/page-run distingue ainda omissão: omissão mantém o valor
anterior. Auto, None, Content vazio e argumento omitido não podem colapsar no
transporte.

O default é Auto. O conteúdo resolvido pertence ao snapshot de cada página e
não é um item visual por si só.

## Localização

Auto usa a chave `page` do catálogo do vanilla ratificado. A implementação deve
reutilizar o owner de localização de L1 e registrar a proveniência da tabela;
não pode inventar traduções nem calibrar strings por saída observada.

Medição direta mínima do catálogo upstream:

- `translations/en.txt:8`: `page`;
- `translations/pt.txt:8`: `página`;
- `translations/es.txt:8`: `página`;
- `translations/fr.txt:8`: `page`;
- `translations/de.txt:8`: `Seite`.

Idiomas já suportados pelo owner de localização devem usar sua entrada
upstream correspondente. Idioma sem entrada recai em inglês, seguindo a
política vigente do owner; não se cria heurística regional nova neste módulo.

## Snapshot e sealing

`PageConfig` guarda o estado não resolvido e o idioma efetivo. No fechamento de
cada página, `Page` recebe `supplement: Content` já resolvido. Page-runs aplicam
o delta local e restauram o estado anterior em LIFO, inclusive quando vazios,
multipágina ou aninhados.

Ao selar o documento, `PageStore` recebe vetores de numbering e supplement com
comprimento igual ao número de páginas e na mesma ordem física. A localização
de um label determina o índice consultado. Não se usa a configuração da página
onde a referência aparece.

## Forma da referência

`RefForm` é enum fechado com `Normal` e `Page`; o default é Normal. Eval e
stdlib aceitam somente as strings `"normal"` e `"page"` e rejeitam qualquer
outro valor com diagnóstico explícito.

Na forma Page:

1. localizar o elemento alvo e sua `Location`;
2. obter a página selada do alvo;
3. exigir numbering nessa página;
4. obter o supplement resolvido dessa mesma página;
5. aplicar a precedência do supplement da referência;
6. produzir link interno para o alvo.

O supplement explícito de `ref` tem precedência: content usa o content; none
suprime o supplement; omitido usa o supplement da página. Callbacks ficam fora
do escopo até existir contrato L0 próprio para execução contextual de
`Supplement::Func`; nunca são convertidas silenciosamente em texto.

O texto é `supplement + NBSP + número` quando o supplement não é vazio. Quando
vazio, é somente o número, sem espaço inicial. A NBSP é morfologia observável da
linguagem; bytes do exporter não são critério.

Se a página alvo não possuir numbering, emitir erro equivalente a `cannot
reference without page numbering` e hint equivalente a habilitar numbering com
`#set page(numbering: "1")`. Mensagem é observável e deve ser medida em teste.

## Fixpoint e pureza

Referências anteriores ao alvo usam o fixpoint/introspector já existente. A
primeira iteração pode não resolver; a iteração convergida deve usar o
`PageStore` selado. L1 não lê arquivos de tradução em runtime, não usa estado
global mutável e não faz I/O.

Owners:

- domínio: `entities/page_supplement.rs`;
- transporte: `layout_types`, `content`, `elements/page_run`, `elements/ref`;
- composição/sealing: `compiler/layout` e `compiler/introspect`;
- referência: `compiler/layout/references.rs`, forma B da ADR-0109.

## Aceitação no nível da linguagem

- quatro estados de entrada preservados até a resolução;
- auto localizado por página e idioma;
- numbering/supplement vêm da página do alvo;
- precedência explícita e NBSP corretas;
- diagnóstico/hint corretos sem numbering;
- referência anterior ou posterior converge para o mesmo texto e destino;
- restauração lexical integral em page-run;
- nenhuma exigência de igualdade de bytes, structs ou passos do algoritmo.

