# Passo 1140.17 — Restaurar a função global `parbreak()`

**Estado:** executado  
**Data:** 2026-08-24  
**Continua:** P1140.16  
**Gate:** ADR-0127 obrigatório antes do código  
**Relatório previsto:** `00_nucleo/diagnosticos/typst-p1140.17-parbreak-global.md`

**Gate confirmado pelo dono em 2026-08-24.** Os hashes finais normalizados são:
`structural/par.md = 848b794d`, `structural.md = 50c093bc`,
`eval.md = 2bb85e70`, `foundations/repr.md = d7574fca` e
`entities/content.md = f17c8a34`. O hash de `par.md` mudou após o gate somente
para incorporar a implementação e os testes autorizados; `repr.md` e `content.md`
foram atualizados antes da correção de representação revelada pelo segundo RED.

## 1. Objetivo

Restaurar a superfície pública ratificada da função global `parbreak()` no
cristalino, reutilizando a semântica já existente de `Content::Parbreak`.

P1140.16 confirmou que:

- vanilla: `repr(type(parbreak)) == "function"`;
- cristalino: erro `unknown variable parbreak`;
- a função vanilla não possui parâmetros;
- o cristalino já representa, avalia por sintaxe, formata e materializa
  `Content::Parbreak`.

O passo não redesenha parágrafos nem adiciona nova variante. Ele expõe como
função a mesma entidade de linguagem que a linha em branco já produz.

## 2. Proveniência da medição inicial

Medição em:

- HEAD: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitado;
- hora: `2026-08-24T13:20:36-03:00`;
- `git diff HEAD --stat`: `75 files changed, 567 insertions(+), 488 deletions(-)`;
- vanilla: `/usr/local/bin/typst`, SHA-256 já confirmado por P1140.16 como
  idêntico ao build ratificado `a51e02804`;
- cristalino: `target/release/typst`, rebuild da working tree em P1140.16.

Comandos e resultados:

```text
/usr/local/bin/typst eval 'repr(type(parbreak))' --format json
→ "function"

/usr/local/bin/typst eval 'repr(parbreak())' --format json
→ "parbreak()"

/usr/local/bin/typst eval 'repr(parbreak(1))' --format json
→ error: unexpected argument

/usr/local/bin/typst eval 'repr(parbreak(foo: true))' --format json
→ error: unexpected argument: foo

target/release/typst eval 'repr(type(parbreak))' --format json
→ error: unknown variable `parbreak`
```

Essas strings são observáveis de linguagem e oracles de teste; não devem ser
transformadas em atalhos mecânicos fora do sistema normal de diagnósticos.

## 3. Fonte vanilla medida

Referência:
`lab/typst-original/crates/typst-library/src/model/par.rs:697-728`.

O vanilla declara `ParbreakElem` sem campos. A documentação estabelece:

1. inicia um novo parágrafo;
2. pode ser produzido por `parbreak()` em código;
3. uma linha vazia em markup é a forma sintática equivalente;
4. múltiplas quebras consecutivas colapsam numa única quebra de parágrafo.

O inventário P1140.16 confirma lista de parâmetros vazia. Não inferir campos,
flags ou argumentos opcionais.

## 4. Estado cristalino medido

Antes de decidir a implementação, confirmar no estado de execução:

- `Content::Parbreak` existe em `entities/content.rs`;
- `SyntaxKind::Parbreak` produz esse conteúdo em `compiler/eval/mod.rs`;
- `repr` já produz `parbreak`/`parbreak()` conforme o contexto vigente;
- layout e realização de parágrafo já tratam o marker;
- `make_stdlib` não registra o binding global;
- não existe outra função nativa concorrente para a mesma unidade.

A causa esperada é ausência do binding/constructor, não ausência da semântica
de parágrafo. Marcar como inferência até o teste RED e a leitura completa dos
donos confirmarem. A inferência é refutada se a chamada exigir novo estado ou
uma variante diferente de `Content::Parbreak`.

## 5. Auditoria L0 obrigatória

Ler integralmente e validar hashes:

1. `00_nucleo/prompts/entities/content.md`;
2. `00_nucleo/prompts/compiler/eval.md`;
3. `00_nucleo/prompts/compiler/stdlib/structural.md`;
4. `00_nucleo/prompts/compiler/stdlib/foundations/repr.md`;
5. L0 que cubra o arquivo stdlib escolhido para a nativa.

O L0 vigente determina `compiler/stdlib/structural/par.rs` como dono das
nativas provenientes de `model/par.rs`; portanto atualizar
`compiler/stdlib/structural/par.md`. Não criar uma fronteira concorrente.

O L0 deve declarar explicitamente:

- assinatura `parbreak() -> Content` sem argumentos;
- retorno sem campos novos, semanticamente equivalente a
  `Content::Parbreak` produzido por markup;
- rejeição de qualquer argumento posicional ou nomeado;
- binding global de kind `function`;
- ausência de mudança na fase de pipeline e no algoritmo de colapso;
- arquivo L1 dono e testes mínimos.

Atualizar `compiler/eval.md` para registrar o binding público. Atualizar outros
L0s apenas se estiverem de fato desatualizados; não tocar documentação por
proximidade nominal.

## 6. Gate ADR-0127

Adicionar `parbreak` ao escopo global altera contrato público. Portanto:

1. escrever/atualizar os L0s;
2. guardar os arquivos;
3. calcular e registrar hashes;
4. **PARAR**;
5. aguardar confirmação explícita do dono;
6. só então escrever teste/código L1.

Não classificar esta mudança como mera tabela interna: a presença do binding é
observável por `type(parbreak)` e pela chamada pública.

## 7. Atomização

A auditoria confirmou a fronteira vigente:

```text
01_core/src/compiler/stdlib/structural/par.rs
```

`native_parbreak` fica ao lado de `native_par`, pois ambas vêm de
`model/par.rs`. O hub estrutural apenas reexporta. Não colocar lógica na
entidade `Content`, não mover layout e não criar despacho dinâmico.

## 8. Testes RED antes do código

Após confirmação do gate, escrever testes que inicialmente falhem:

1. `repr(type(parbreak)) == "function"`;
2. `repr(parbreak()) == "parbreak()"`;
3. `parbreak(1)` produz `unexpected argument`;
4. `parbreak(foo: true)` produz `unexpected argument: foo`;
5. `parbreak()` entre dois trechos cria dois parágrafos, como uma linha vazia;
6. duas chamadas consecutivas colapsam semanticamente como duas linhas vazias;
7. uso dentro de `for` preserva a forma documentada pelo vanilla;
8. o binding via `std.parbreak` possui o mesmo kind e comportamento do global.

Separar testes de constructor/eval dos testes de layout existentes. Não exigir
igualdade Rust de árvores nem bytes PDF.

## 9. Implementação mínima autorizável

Depois do gate e do RED:

1. criar a função nativa atomizada;
2. validar ausência total de argumentos pelo mecanismo normal de `Args`;
3. retornar `Value::Content(Content::Parbreak)`;
4. reexportar no hub stdlib;
5. registrar em `make_stdlib`:

```text
scope.define("parbreak", Value::Func(Func::native("parbreak", native_parbreak)))
```

6. ressellar hashes de linhagem;
7. executar RED→GREEN.

Não adicionar campos a `Content`, não alterar `Parbreak` sintático e não
introduzir parâmetros ausentes no vanilla.

## 10. Controles de não regressão

- linha vazia em markup continua produzindo quebra de parágrafo;
- `linebreak()` e `pagebreak()` permanecem inalterados;
- `repr(Content::Parbreak)` mantém a forma canônica;
- colapso de múltiplas quebras continua no dono existente;
- `Content::Parbreak.is_empty() == false` e `plain_text() == "\n"` permanecem;
- nenhum binding global existente é substituído.

## 11. Aceitação

O passo fecha quando:

1. L0 foi atualizado antes do código e confirmado no gate ADR-0127;
2. RED público foi observado e registrado;
3. `parbreak` é função global e em `std`;
4. chamada sem argumentos retorna a quebra existente;
5. argumentos posicionais e nomeados reproduzem os diagnósticos relevantes;
6. equivalência com linha vazia e colapso consecutivo passam no nível da
   linguagem;
7. inventário P1140.16 reclassifica `parbreak` de `MISSING_BINDING` para
   `UNVERIFIED_METADATA` ou `MATCH`, conforme a capacidade do instrumento;
8. probes P1140.16 passam para `parbreak`;
9. passam testes focados e suíte do núcleo;
10. `cargo build --workspace` passa;
11. `crystalline-lint .` termina com exit 0 e hashes válidos.

## 12. Relatório

Escrever
`00_nucleo/diagnosticos/typst-p1140.17-parbreak-global.md` com:

- proveniência antes/RED/GREEN;
- contrato vanilla medido;
- L0s e hashes alterados;
- confirmação do gate;
- unidade dona final;
- testes e comandos;
- resultado do probe e inventário;
- confirmação de que nenhuma semântica nova de parágrafo foi criada.

## 13. Fora de escopo

- função global `page`;
- tipo global `path`;
- feature `html`;
- metadados genéricos de assinatura em `Func`;
- novos campos ou modos de `Parbreak`;
- mudanças de spacing, line breaking ou paginação;
- igualdade de bytes PDF.

## 14. Resultado da execução

O passo foi executado em fluxo RED→GREEN. O primeiro RED observou
`unknown variable parbreak`; depois do binding, o segundo RED revelou que o
cristalino representava o conteúdo como `parbreak`, enquanto o vanilla produz
`parbreak()`. O L0 de `repr` foi atualizado antes dessa correção.

Resultado final:

- `parbreak` está disponível globalmente e por `std` como função;
- `parbreak()` retorna o `Content::Parbreak` já existente;
- argumentos posicionais e nomeados produzem, respectivamente,
  `unexpected argument` e `unexpected argument: foo`;
- `repr(parbreak()) == "parbreak()"`;
- inventário: `MISSING_BINDING` caiu de 4 para 3 e
  `UNVERIFIED_METADATA` subiu de 176 para 177;
- probes: 16 de 28 iguais, contra 15 de 28 em P1140.16;
- suíte L1: 5.162 testes aprovados, zero falhas;
- `cargo build --workspace`, build release do binário e
  `crystalline-lint .` concluíram com sucesso.

Proveniência final: HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`,
working tree não commitada, medição em `2026-08-24T13:31:30-03:00`, com
`83 files changed, 734 insertions(+), 505 deletions(-)` no `git diff HEAD --stat`.
