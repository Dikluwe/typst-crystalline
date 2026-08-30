# P1165 — auditar e nuclear a superfície pública `html` feature-gated

**Data:** 2026-08-25
**Estado:** `EXECUTADO — PARADO NO GATE ADR-0127`
**Baseline cristalina:** commit `30a6f11bc`
**Baseline de linguagem:** vanilla ratificado `a51e02804`
**Gate:** ADR-0127 obrigatório antes de criar feature, módulo ou contrato público

## Objetivo

Auditar a superfície pública `html` como feature experimental da linguagem,
separando-a do exporter HTML já existente. Medir os defaults e a superfície
com a feature desligada/ligada no vanilla ratificado, inventariar o que o
cristalino já possui em eval, CLI, pipeline e L3, redigir os L0s necessários e
parar no gate ADR-0127 antes de qualquer código.

Este passo é de auditoria e nucleação. Não implementa `html.elem`, tags
tipadas, `html.frame`, feature flags, DOM, CSS, MathML nem amplia o exporter.

## Estado conhecido que deve ser revalidado

O handoff pós-P1140 classifica `html` como frente feature-gated separada. Desde
P1137-X-002, porém, o cristalino já contém:

- `OutputFormat::Html` e resolução por `.html`/`--format html` em L2;
- `EvalTarget::Html` em L1;
- pipeline semântico `compile_to_html_string` em L3;
- exporter HTML inicial para texto, parágrafo, heading, strong, emph e
  linebreak;
- dispatch L4 e warning experimental;
- `info` que anuncia HTML como disponível.

Isso legitima exportar o subconjunto atual, mas não legitima automaticamente
o binding global `html` nem um sistema público de features. A ADR-0128 está
marcada `PROPOSTO`; auditar também se o seu estado precisa de ratificação ou
adendo antes de ampliar o contrato.

## Proveniência obrigatória

Antes de qualquer classificação ou número, registar:

```text
date --iso-8601=seconds
git rev-parse HEAD
git status --short
sha256sum /usr/local/bin/typst ./target/debug/typst
/usr/local/bin/typst --version
./target/debug/typst --version
```

Não usar `--version` como prova do vanilla; o alvo é o pin `a51e02804`. Se a
working tree não estiver limpa, listar `git diff HEAD --stat` e repetir a
proveniência quando o estado mudar.

## Ordem obrigatória

### 1. Ler decisões e L0s vigentes

Ler integralmente, sem listar ou varrer `00_nucleo/context/` e
`00_nucleo/materialization/`:

- `AGENTS.md`;
- `00_nucleo/diagnosticos/typst-auditoria-handoff-pos-p1140.md`, seção 4.9;
- `00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md`;
- `00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md`;
- `00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md`;
- `00_nucleo/adr/typst-adr-0128-html-target-semantico.md`;
- `00_nucleo/prompts/compiler/eval.md`;
- `00_nucleo/prompts/shell/cli.md`;
- `00_nucleo/prompts/infra/pipeline.md`;
- `00_nucleo/prompts/infra/export/html.md`;
- `00_nucleo/prompts/wiring.md`;
- L0s apontados pelos headers dos ficheiros adicionais encontrados.

Antes de propor arquitetura, registrar o que cada documento já decide e as
lacunas reais. Não reutilizar diagnósticos antigos que diziam “HTML ausente”
como descrição do estado atual.

### 2. Medir a matriz feature × comando no vanilla

Executar no vanilla ratificado, preservando comando exato, stdout, stderr e
exit code:

| Feature | Comando/target | Finalidade |
|---|---|---|
| desligada | `eval` de `type(html)` e `repr(html)` | ausência e diagnóstico default |
| ligada | as mesmas probes com `--features html` | existência/tipo do módulo |
| desligada | `compile --format html` de documento mínimo | relação formato ↔ feature |
| ligada | o mesmo compile com `--features html` | caminho autorizado |
| desligada/ligada | `eval target()` com target paged/html | target e feature são eixos distintos? |
| ligada | `info --format json` | feature anunciada e estado efetivo |

Descobrir a posição sintática correta de `--features html` pelo `--help`; não
normalizar o comando para fazer a sonda passar. Repetir a matriz no cristalino
e registrar se a flag é ausente, ignorada ou implementada.

Sondas mínimas de documento:

```typst
Hello, parity.
= Heading
This is *strong* and _emphasized_.
#linebreak()
```

Comparar morfologia DOM e diagnósticos. Contagem de bytes só pode ser prova de
reprodução quando acompanhada da proveniência; bytes não substituem a análise
semântica de tags/estrutura (ADR-0107).

### 3. Inventariar a superfície pública habilitada

Com a feature ligada no vanilla, enumerar o namespace completo sem escolher
um subconjunto antes da medição. Usar reflexão pública quando disponível e
confrontar com a fonte ratificada:

- `typst-library/src/lib.rs: binding html + Feature::Html`;
- `typst-library/src/routines.rs: html_module`;
- `typst-html/src/lib.rs: module()`;
- `typst-html/src/typed.rs: tags tipadas`;
- `typst-html/src/dom.rs`, `fragment.rs`, `rules.rs` e `document.rs` somente
  para classificar owners/observáveis encontrados.

Para cada binding público, registrar:

```text
nome
tipo (`type`/`func`/`element`/módulo)
assinatura pública
posicionais e named args
defaults
content body
repr/type/fields observáveis
target permitido
diagnóstico fora de target
efeito no HTML emitido
```

Cobrir obrigatoriamente as famílias:

- primitivas `html.elem` e `html.frame`;
- tags tipadas expostas pelo módulo (`div`, `span`, `p`, headings, listas,
  links, mídia, tabela, metadata e demais entradas realmente encontradas);
- atributos genéricos versus atributos tipados;
- conteúdo inline/block e regras de whitespace;
- `html.script`/`html.style` e restrição de body, se confirmada;
- interação com `target()`, show/set rules e `repr`;
- erros de tag/atributo/valor inválido.

Não inferir a API a partir dos nomes HTML conhecidos: a fonte/sonda do Typst é
o contrato.

### 4. Auditar o estado cristalino e os owners

Inventariar com `rg`, registrando `file:line`:

- modelo de features em CLI, world/library, scope e eval;
- construção do scope global e ponto onde `html` seria gated;
- `EvalTarget` e propagação paged/html;
- `target()` e qualquer comentário/default desatualizado;
- `Content` capaz ou incapaz de representar nós/atributos HTML;
- exporter `03_infra/src/export/html.rs` e variantes aceites/rejeitadas;
- `compile_to_html_string` e dispatch L4;
- `OutputFormat::Html`, help, `info.features.html` e flags de compile/watch;
- introspecção/position/anchor e guarda `check_html_depth` relevantes ao
  target, sem puxar automaticamente essas dívidas para o primeiro corte.

Produzir uma tabela:

```text
superfície vanilla | cristalino | MATCH/PARTIAL/ABSENT/EXTRA | owner | L0 vigente
```

Separar explicitamente:

1. feature gate da linguagem;
2. binding/módulo público `html`;
3. representação L1 de conteúdo HTML;
4. expansão/documento HTML;
5. serialização L3;
6. flags/dispatch L2/L4;
7. introspecção/posição HTML.

### 5. Classificar antes de decidir

Aplicar ADR-0107/0108:

- existência do binding sob feature, assinatura, defaults, diagnóstico,
  estrutura semântica do DOM e saída textual HTML são observáveis;
- crates, traits, vtables, generics de target, estruturas Rust e algoritmo de
  expansão são mecânica;
- o fato de `OutputFormat::Html` já existir não prova que a feature deve ficar
  ligada por defeito;
- o fato de `info` anunciar HTML não prova que o módulo público está ativo;
- o fato de o vanilla usar `typst-html` não autoriza importar `lab` nem copiar
  sua topologia.

Marcar cada inferência e a evidência que a refutaria. Em particular, medir se
selecionar output HTML habilita implicitamente a feature ou se os eixos são
independentes.

### 6. Definir cortes verticais, sem implementar

Só depois do inventário completo, propor a menor sequência coerente. A
proposta deve considerar, sem escolher antecipadamente:

- **Corte A:** modelo de features + gate e diagnóstico, ainda sem binding;
- **Corte B:** representação L1 mínima + `html.elem`;
- **Corte C:** tags tipadas fundamentais guiadas por tabela;
- **Corte D:** integração no exporter/expansão;
- cortes posteriores para frame, whitespace, listas/tabelas, mídia, MathML,
  introspecção e positions.

Cada corte deve declarar o que fica incompleto e qual passo o completa,
conforme a regra de divisão de constructor/funcionalidade. Não anunciar
`MATCH` global a partir do documento sentinela.

### 7. Redigir os L0s necessários e parar

Criar ou atualizar primeiro as especificações, conforme os owners medidos. A
lista candidata — a auditoria pode refiná-la — é:

- novo `00_nucleo/prompts/compiler/stdlib/html.md` para o módulo público;
- `00_nucleo/prompts/compiler/eval.md` para feature gating e target;
- novo L0 de entidade se for necessário representar nó/atributos HTML em L1;
- `00_nucleo/prompts/shell/cli.md` para `--features` e defaults;
- `00_nucleo/prompts/infra/pipeline.md` e
  `00_nucleo/prompts/infra/export/html.md` para o corte de expansão/export;
- `00_nucleo/prompts/wiring.md` para propagação da configuração;
- ADR-0128, se a auditoria exigir ratificação/adendo da decisão proposta.

Os L0s devem incluir medição `file:line` antes da decisão, matriz de defaults,
tipos/assinaturas, diagnósticos, invariantes, scope-outs e critérios de língua.
Não ressellar headers nem escrever testes/código neste passo.

Então **PARAR NO GATE ADR-0127** e pedir aprovação explícita do dono, porque a
materialização pode envolver simultaneamente:

- enum/estrutura pública de features;
- nova flag pública `--features`;
- novo binding/módulo público `html`;
- novas entidades/variantes públicas L1;
- mudança do comportamento/default atualmente anunciado para HTML.

## Entrega obrigatória do P1165

1. proveniência completa das duas toolchains;
2. matriz feature × comando × target;
3. namespace público vanilla integral, agrupado por família;
4. inventário cristalino `file:line` e matriz de gaps;
5. classificação língua versus mecânica;
6. proposta de cortes verticais e dependências;
7. L0s redigidos/atualizados, ainda não ressellados;
8. decisão explícita sobre a situação da ADR-0128;
9. lista de contratos públicos que exigem aprovação;
10. parada sem testes RED, sem código e sem staging/commit.

## Critérios de aceitação

- vanilla sempre medido com e sem `--features html`;
- export HTML e módulo público `html` permanecem conceitos separados;
- namespace completo é inventariado antes de selecionar o primeiro corte;
- nenhum default é inferido pelo estado atual do cristalino;
- nenhum código L1–L4 é escrito antes do L0 e do gate;
- pastas restritas não são acessadas;
- a working tree ao final contém somente o passo/diagnóstico/L0s desta
  auditoria, todos fora de staging;
- o handoff termina com uma pergunta objetiva de aprovação ADR-0127.

## Próximo passo após aprovação

Somente depois da ratificação do dono, escrever P1166 para ressellar hashes,
produzir testes RED do primeiro corte aprovado e materializá-lo. Se o dono
rejeitar a ativação por defeito, P1166 deve preservar a ausência de `html` sem
feature e testar o diagnóstico medido como parte do contrato.
