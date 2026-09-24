# Prompt L0 — `infra/integration_tests` — suíte E2E L3
Hash do Código: 651faa9c

**Camada:** L3, somente `#[cfg(test)]`
**Ficheiro proprietário:** `03_infra/src/integration_tests.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0129

## Responsabilidade

A suíte exercita o pipeline real por fronteiras que mocks de L1 não cobrem:
`SystemWorld`, filesystem temporário, eval, introspecção, layout e export. Ela
legitima somente o harness e os testes do consumer; módulos produtivos mantêm
seus próprios owners.

Helpers test-only criam e removem diretórios temporários, avaliam fontes,
atravessam frames e compilam documentos. I/O e world concreto são permitidos
neste consumer L3 e não se transferem para L1.

## Observáveis

Testes podem verificar valores de eval, morfologia e layout, warnings,
diagnósticos e estruturas observáveis dos formatos exportados. Igualdade de
bytes ou estrutura interna só é contrato quando ela própria é o observável do
formato.

## P1484 — gate `bundle` no pipeline real

### Medição anterior à decisão

Em HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working tree
não commitada descrita no Passo 1484 §6, o teste legado
`03_infra/src/integration_tests.rs:1760-1774` chama
`#asset("logo.png")` sem feature, faz `unwrap()` do eval e exige PDF
sem frames. Após P1484, `cargo test --workspace` mostra esse único
teste L3 a falhar: eval produz o diagnóstico gated
`cannot access variable \`asset\` because the \`bundle\` feature is not enabled`.
O teste mede uma extensão cristalina P397, não paridade com o vanilla
pinado `586e1bd43`; P1484 confirmou que o placeholder não é público.

### Decisão

Substituir somente esse teste legado por uma sonda E2E que exige o
diagnóstico gated no eval através de `SystemWorld`, com mensagem e hints
exatos. Não chegar a layout/PDF quando o eval falha. O caminho ativo e
o constructor real continuam abertos até P1485. Os demais testes de
export/layout não mudam.

## Coordenadas globais de `FrameItem`

Helpers que comparam geometria carregam a transformação afim acumulada do
ancestral até o item:

- ao entrar em `Group`, compõem a translação de `Group.pos`, a
  `Group.matrix` e o transform ancestral na mesma ordem do exporter;
- grupos aninhados compõem transitivamente;
- pontos de `Text`, `TextShaped`, `Glyph`, `Image` e `Shape`, além dos
  extremos de `Line`, são projetados uma única vez no referencial global;
- `Semantic` é envelope transparente e `Link` preserva sua convenção.

Travessias que apenas contam ou classificam itens podem observar variantes
locais. Toda asserção geométrica usa coordenadas globais. É proibido compensar
o transporte alterando expectativas, fixtures ou layout produtivo.

## Aceitação

- o módulo só compila sob `cfg(test)`;
- helpers permanecem privados ao harness;
- temporários são removidos;
- testes passam sem mudar contratos produtivos para acomodar mecânica do
  observador.

## P1466 — fixtures de cor e cercas invertidas

Fixtures geométricas usam valores públicos de cor (`red`, `blue`, `black`),
não strings com esses nomes. A retificação deve permitir que os testes de
rect, ellipse e rotate alcancem as asserções originais de operadores,
geometria e transformação; não trocar os observáveis nem aceitar erro de cast.

O witness `sec_22.typ` preserva os dois pares de brackets, normal e invertido.
Coletar `base_char`, `glyph_id` e avanço dos quatro itens e exigir mesma
variante vertical por caractere quando o papel posicional muda. Esse teste
protege a correção do owner `compiler/math/layout/delimited.md` e não requer
estrutura `MathDelimited`.

## P1485 — corpus E2E do target bundle (PROPOSTO)

### Medição anterior à decisão

O teste P1484 deste consumer confirma somente o gate sem
feature. A sonda vanilla P1485 em diretório isolado produziu
asset `0 255 10`, PDF/SVG mistos, HTML condicionado à
feature, e erros de duplicata, top-level solto e path inválido.
Sem target bundle, `#asset` e `#document` diretos têm
diagnósticos próprios. Referências exatas e proveniência estão
em `00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md`.

### Obrigação dos testes

Antes da implementação, escrever RED via `SystemWorld` para
arquivo binário e string UTF-8, path aninhado/normalizado,
duplicatas entre dois assets e asset+document, bundle vazio,
PDF/SVG/PNG/HTML de documento, formato explícito, formato
desconhecido, HTML sem feature, top-level solto, fora do target
e ausência de feature. GREEN compara nomes e payloads de
asset exatamente; documentos com critérios de linguagem e
formato, não bytes PDF/SVG por igualdade mecânica. A suíte
não deixa arquivos fora de diretórios temporários e não
aceita placeholder sem dados como sucesso.
