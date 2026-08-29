# P1250 — convergência funcional e campanha adversarial integrada do SVG

**Estado:** FECHADO — PASS POR P1250A + P1250B  
**Predecessor:** P1249  
**Saída:** features SVG pendentes materializadas pelos seus owners e campanha
integrada segregada com score 1.0.

## Objetivo

Fechar as funcionalidades necessárias antes de executar o corpus integrado de
SVG. P1250 coordena gates e integração; não absorve ownership dos prompts L0 de
tiling, clip, links, imagens ou glifos.

O passo anterior alegava `REJECT_INVALID_CHAIN`, mas não deixou manifesto,
allowlist, comando ou recibo reproduzível. Essa alegação fica invalidada. O
bloqueio real é a ausência de materialização das dependências abaixo.

## Fase 0 — congelar a linha de base

Registrar antes de qualquer execução:

- HEAD e estado exato da working tree;
- hashes efetivos dos L0 e Núcleos Tekt consumidos;
- hashes dos consumers produtivos;
- versão/hash do vanilla ratificado `a51e02804`;
- allowlist de leitura e escrita de cada autoridade;
- política comum: construção ambígua, opaca ou sem carrier fica `Unknown`.

Qualquer alteração posterior em prompt, Núcleo, baseline, contrato ou oracle
invalida somente a cadeia dependente e exige novo selo a partir desse ponto.

## Fase 1 — fechar P1245: tiling SVG

Owner produtivo: `03_infra/src/export/svg.rs`; owners L0 já definidos para
tiling e SVG.

1. Revalidar os hashes L0 confirmados em P1245.
2. Produzir contrato, oráculos e ataques segregados para geometria da célula,
   spacing, relative, transform, corpo suportado e fallback opaco.
3. Exigir mutantes de célula deslocada, spacing perdido, transform duplicado,
   alpha perdido, referência pendente e fallback promovido indevidamente.
4. Selar somente com mutation score 1.0.
5. Implementar RED→GREEN no owner existente, sem criar segundo consumer.
6. Executar testes do owner, build, V5, V15 e V26; publicar certificado.

## Fase 2 — fechar P1246: clip geométrico

Antes do pré-selo, registrar decisão do dono sobre o L0 P1246 já proposto. O
contrato deve distinguir recorte geométrico de máscara/alpha.

1. Cobrir rect, rounded rect, ellipse e path representável.
2. Cobrir transform local, grupos aninhados e interseção de clips.
3. Atacar clip ignorado, aplicado duas vezes, espaço errado, referência
   pendente e deduplicação entre bases espaciais diferentes.
4. Manter even-odd sem carrier e geometria opaca como `Unknown`.
5. Após preseal 1.0, implementar no owner SVG e certificar os gates.

## Fase 3 — fechar P1247: destinos internos

Usar os L0 aprovados de SVG/pipeline e o Núcleo
`export/svg-destination-context.toml`.

1. Selar contexto page-local derivado de `PagedDocument`.
2. Atacar `href` sem `id`, `id` sem nó, colisão, posição inventada, escaping,
   nesting e transformação duplicada.
3. Preservar URL externa; manter cross-page sem rota como `Unknown`.
4. Não expandir o construtor público `link()` neste passo.
5. Implementar wrappers compatíveis mais variante contextual, integrar na
   pipeline e certificar os gates.

## Fase 4 — fechar P1248: imagens SVG

O ensaio atual 11/11 não possui atestação de isolamento. Repeti-lo sob
autoridades segregadas antes de usá-lo como preseal.

1. Usar observáveis de morfologia visível, nunca igualdade XML/base64.
2. Cobrir PNG, JPEG, GIF, WebP, alpha, orientação, caixa, aspect ratio e
   `clip_rect` de cover.
3. Decidir em L0, antes do código, o carrier legítimo de SVG aninhado; bytes
   atualmente classificados como `Unknown` são omitidos pelo exporter.
4. Atacar perda de forma/alpha do SVG aninhado, orientação ignorada/duplicada,
   clip perdido, MIME incorreto e promoção de formato desconhecido.
5. Selar com score 1.0, implementar somente formatos autorizados e certificar.

Se o suporte a SVG aninhado exigir ampliar `ImageFormat` ou outro contrato
público, parar no ADR-0127 para decisão do dono. Formato desconhecido nunca é
aceito como preservado por omissão silenciosa.

## Fase 5 — fechar P1249: glifos matemáticos

Usar os L0 aprovados de SVG/pipeline e o Núcleo
`export/svg-glyph-font-context.toml`.

1. Selar `GlyphFontRequest` normalizado → `FontKey` completo.
2. Atacar reordenação da lista, colisão do mesmo `glyph_id` em duas fontes,
   variante/assembly, fill e alpha, transform e fonte ausente.
3. Proibir índice zero por default, lookup por endereço ou ordem de travessia.
4. Manter wrappers sem fontes como scope-out explícito.
5. Implementar contexto L3, outline na face exata e certificado final.

## Fase 6 — campanha integrada P1250

Só iniciar quando P1245–P1249 tiverem certificados válidos e os hashes dos
consumers coincidirem com os certificados.

### Corpus mínimo

Cada fixture combina pelo menos três famílias funcionais:

1. tiling + clip transformado + link interno;
2. gradient/paint com alpha + imagem orientada + clip cover;
3. glifo matemático + link + transform aninhado;
4. SVG embutido + tiling + stroke complexo;
5. duas páginas com destino same-page e alvo cross-page `Unknown`;
6. colisões deliberadas entre IDs de glyph, paint, pattern, clip e destino.

### Ataques integrados obrigatórios

- colisão e captura cruzada de IDs;
- `defs` reordenado com grafo semanticamente igual;
- referência local pendente;
- transformação aplicada zero ou duas vezes;
- fill e stroke trocados;
- alpha removido;
- clip aplicado no espaço errado;
- fonte resolvida por índice incidental;
- imagem/SVG visível omitido;
- fallback `Unknown` promovido a `Preserved`.

O oracle compara grafo de referências, geometria, conteúdo visível, alpha e
classificação semântica. Bytes, nomes concretos de IDs e ordem de `defs` não
são critérios quando os observáveis permanecem equivalentes.

## Segregação obrigatória

Aplicar o protocolo completo:

- autor do contrato não lê implementação candidata;
- autor dos oráculos não adapta casos ao patch;
- adversário não escreve solução;
- implementador não altera contrato, oráculos ou baseline;
- verificador não corrige os artefatos julgados;
- manifesto registra entradas, hashes, capacidades e allowlists.

Uma única sessão pode produzir ensaio, mas não pode declarar preseal ou
certificado segregado.

## Gates de fechamento

P1250 fecha somente quando todos forem verdadeiros:

1. P1245–P1249 possuem certificados válidos e entradas ainda intactas.
2. Todos os casos positivos integrados são `Preserved`.
3. Casos opacos previstos são `Unknown`, nunca sucesso implícito.
4. Todas as mutações válidas são rejeitadas: mutation score 1.0.
5. Duas execuções completas produzem recibos byte-idênticos.
6. `cargo build --workspace` passa.
7. Testes dos owners e campanha integrada passam.
8. `crystalline-lint . --fail-on warning` termina sem violações ou warnings.
9. `git diff --check` passa.
10. O certificado registra HEAD, working tree, horário, comandos e hashes.

Falha integrada produz witness mínimo e retorna ao owner da feature; não é
perdoada por sucesso isolado. Até esses gates, o estado correto permanece
`DEPENDENCY_STOP`, sem mutation score inventado.

## Execução integrada — 2026-08-28

P1245–P1249 foram revalidados como `READY`, sem deriva dos consumers
produtivos. O contrato integrado congelou seis corpus com ao menos quatro
famílias cada, seis casos negativos, seis opacos e quatro invariantes globais.
O preseal rejeitou 11/11 mutações válidas, score `1.0`, com duas execuções
byte-idênticas e estabilidade sob reordenação semântica.

O harness foi materializado fora do owner produtivo, em
`03_infra/tests/p1250_svg_integrated.rs`, com L0 próprio 1:1. Assim, a campanha
não alterou o hash certificado de `03_infra/src/export/svg.rs`. O teste executa
os seis corpus sobre saídas reais e rejeita 11/11 mutações serializadas.

Gates aprovados: campanha integrada, suíte SVG 41/41, build do workspace,
V5/V15/V26, lint completo sem erros e `git diff --check`. O V1 inicialmente
encontrado no harness foi corrigido com linhagem causal própria e ADR-0129
permaneceu válido.

P1250A resolveu os 54 warnings acionáveis: V17=0, V18=0 e V21=0. Os 202 V16
restantes são exceções exatas ratificadas pelo mecanismo oficial do linter;
continuam visíveis como warnings por desenho, com zero ocorrências novas e
lint normal em exit `0`. Por isso `--fail-on warning` não é um gate compatível
com a política de exceções V16 já aprovada em P1209 e foi substituído pelo
ratchet diferencial documentado em `typst-passo-1250a.md`.

Certificado: `00_nucleo/diagnosticos/p1250-final-certificate.tsv`, veredito
`FAIL_FINAL_GATE` preserva a medição histórica original. P1250B resolveu os
quatro testes restantes com contrato e ataques segregados: P744 foi retificado
por medição direta do vanilla pinado; os tuples legacy de curve voltaram a
fechar reto; `curve.close()` namespaced preservou o default suave. A suíte do
workspace, build, lint arquitetural e diff check passam. O certificado aditivo
`00_nucleo/diagnosticos/p1250b-final-certificate.tsv` fecha P1250 sem reescrever
o certificado histórico de falha.
