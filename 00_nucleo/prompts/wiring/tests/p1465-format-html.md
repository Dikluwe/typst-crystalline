# Prompt L0 — integração de `format.html`

Hash do Código: 58a60d1f

**Estado:** CONTRATO DE TESTE P1465 MATERIALIZADO; gate ADR-0127 satisfeito.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11

**Camada:** L4 (testes de integração)
**Alvo planejado:** `04_wiring/tests/p1465_format_html.rs`
**Consumer futuro exclusivo:** `04_wiring/tests/p1465_format_html.rs`
**Vanilla ratificado:** `586e1bd43fae6c9a973218163d3165c53ab8d16d`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129

## Responsabilidade

Executar o binário cristalino como processo externo e provar somente a
superfície pública `format.html`, seu gate, a reutilização dos 114 membros e o
transporte vertical de `pretty`. O teste não importa módulos privados, não
inspeciona identidade de ponteiro ou ordem interna e não declara paridade HTML
geral.

## Matriz de features

Executar os perfis:

| Perfil | Features explícitas |
|---|---|
| `default` | nenhuma |
| `a11y-extras` | `a11y-extras` |
| `html` | `html` |
| `html+a11y` | `html,a11y-extras` |

Sem `html`, acesso direto, alias de `format`, `std.format.html` e set rule
falham com exit de erro, stdout vazio, mensagem integral de feature desativada,
dois hints na ordem vanilla e span somente no field. Um argumento `panic` não
pode executar antes do gate. Target, exporter e `a11y-extras` não habilitam a
feature.

Com `html`, verificar:

- `type(format.html) == function`, `repr(format.html) == "html"` e
  `pretty == false`;
- `format.html()` falha com `cannot be constructed manually`;
- alias lexical e `std.format.html` preservam tipo, nome e namespace;
- `elem`, `frame` e as 112 tags existem, são funções, têm nomes curtos e
  comparam iguais aos respectivos valores `html.*`;
- todos os 114 membros permanecem descobertos por
  `format.html.with(pretty: true)`;
- field desconhecido conserva o diagnóstico ordinário de função, sem ser
  confundido com gate.

## Set rule e opção documental

Cobrir `pretty: true`, `pretty: false`, regra vazia, true→false, false→true e
true→vazia. Named desconhecido, inteiro, positional e duplicado produzem as
mensagens medidas. Com feature ativa, expressão de argumento é avaliada antes
do erro de schema; sem feature, o gate precede toda avaliação.

Regra root controla o output HTML. A mesma regra dentro de ao menos dois
containers estruturalmente distintos falha com:

```text
format set rules are not allowed inside of containers
```

Verificar a precedência pública:

| Documento | `--pretty` | Resultado |
|---|---|---|
| ausente | ausente | compacto |
| true | ausente | pretty |
| true | false | compacto |
| false | true | pretty |

Compacto e pretty devem ter DOM/escaping semanticamente idênticos e diferir
somente no whitespace estrutural contratado. Não usar hash ou bytes integrais
como oráculo. O SVG inline de `html.frame` recebe a mesma decisão final do
documento externo.

## Controles de fronteira

- `format.svg`, `format.png`, `format.pdf` e seus defaults permanecem;
- o módulo global `html` e os 114 membros não são reconstruídos nem alterados;
- gates `pdf.*`/`format.pdf.*` continuam ortogonais a `html`;
- PDF mantém a resolução vigente até P1466; PNG ignora pretty; SVG usa sua
  própria opção documental;
- os modos de serialização HTML Crystalline/Vanilla conservam seus escapes;
- bundle, novas tags, target novo e paridade HTML geral ficam fora.

Testes com subprocesso têm timeout finito. Targets, caches, fixtures, outputs
e logs descartáveis usam exclusivamente uma raiz de tarefa em `/dev/shm`, sem
fallback ao disco. Execução serial, limite por processo e watchdog respeitam o
teto de 32 GiB; falta de espaço, OOM ou timeout é `Unknown`, nunca sucesso.

## Gate satisfeito

A confirmação humana explícita posterior ao commit causal
`e300314e2f87e90e543c4923a8cae97f9ad978a8` foi recebida em 2026-09-20. O
consumer exclusivo foi criado depois do gate; o primeiro RED teve 0/4 testes
verdes e a implementação produtiva levou a matriz a 4/4.
