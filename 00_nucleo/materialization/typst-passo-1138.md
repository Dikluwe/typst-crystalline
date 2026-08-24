# Passo 1138 — Ampliar o denominador de paridade além dos smoke tests

**Data:** 2026-08-23  
**Baseline herdada:** P1137 fechado com 10 `MATCH` em 10 sentinelas  
**Vanilla ratificado:** `upstream/main a51e02804`  
**Natureza:** medição e infraestrutura em `lab/`; sem correções L1–L4

## 1. Objetivo

Transformar os limites explicitamente declarados pelo P1137 em comparações
profundas e reproduzíveis. Um `MATCH` de presença de artefacto não pode ser
promovido implicitamente a igualdade semântica, morfológica, geométrica ou
visual.

O resultado deste passo é:

1. comparadores próprios por observável;
2. um denominador maior, com IDs estáveis;
3. uma baseline que liste integralmente todos os resultados não-MATCH;
4. um backlog priorizado para passos de correção posteriores.

## 2. Escopo

### P1138-A — diagnósticos públicos

- Comparar exit code, classe, mensagem pública e span.
- Normalizar somente o prefixo absoluto do path, preservando ficheiro, linha e
  coluna.
- Cobrir erro sintático, nome desconhecido e erro numa dependência importada.
- Um diagnóstico não é `MATCH` apenas porque ambos os processos falham.

### P1138-B — valores tipados e superfície da linguagem

- Usar DTO tipado para `none`, bool, int, float, string, bytes, array e dict.
- Extrair inventário observável de globais, módulos, funções, elementos e
  métodos.
- Para cada callable medido, registrar argumentos posicionais/nomeados,
  obrigatoriedade e defaults públicos.
- Não usar nomes de funções Rust, LOC ou presença de ficheiro como proxy.

### P1138-C — árvores de exportação semântica

- SVG: comparar árvore, ordem, tags e atributos públicos; excluir apenas
  identificadores comprovadamente voláteis.
- HTML: comparar árvore semântica para texto, parágrafo, heading, strong,
  emph, linebreak e escape.
- Distinguir `PARTIAL` de `ABSENT`: backend presente com subconjunto explícito
  nunca é ausência total.

### P1138-D — layout, raster e PDF observável

- Geometria: páginas, frames, posições e dimensões em pt, com tolerância
  declarada por campo e justificada por fonte.
- Raster: dimensões, alpha e diferença de pixels; produzir imagem de diferença
  apenas como artefacto diagnóstico.
- PDF: comparar páginas, caixas, fontes, imagens, links e texto extraído; não
  comparar bytes brutos nem ordem mecânica de objetos.
- Cobrir ao menos texto simples, RTL com duas quebras, heading, imagem e uma
  expressão matemática conhecida como sensível.

### P1138-E — relatório e priorização

- Emitir contagens por eixo e estado com denominador explícito.
- Listar integralmente `DIFF`, `ABSENT`, `PARTIAL`, `ERROR` e `UNMEASURED`.
- Classificar língua versus mecânica conforme ADR-0107.
- Priorizar primeiro divergências que alteram documentos, depois contratos
  parciais, capacidades ausentes, diagnósticos/CLI e formatos.
- Cada número decisório registra HEAD, estado da working tree e horário.

## 3. Ordem de execução

1. Escrever testes RED do schema e de cada comparador novo.
2. Implementar `diagnostic`, `typed_value`, `semantic_tree`, `geometry`,
   `raster` e `pdf_observables` separadamente.
3. Adicionar fixtures mínimas e independentes.
4. Executar os mesmos casos nos dois binários.
5. Registrar a baseline sem corrigir produção.
6. Produzir o backlog dos passos 1139+ a partir das divergências medidas.

## 4. Travas

- Não ler `00_nucleo/context/` nem `00_nucleo/materialization/` sem path
  completo fornecido pelo dono.
- Não alterar L1–L4 durante este passo.
- Uma divergência descoberta não é corrigida dentro do runner.
- Nenhuma tolerância pode ser escolhida apenas para converter `DIFF` em
  `MATCH`; toda tolerância tem unidade, razão, fonte e teste-limite.
- Representação Rust, bytes PDF e ordem interna de objetos são mecânica, salvo
  quando constituírem o observável público do caso.
- Qualquer correção de produção posterior começa por auditoria do L0; aplicar
  o gate da ADR-0127 quando houver contrato público, default, fase de pipeline
  ou quebra de compatibilidade.

## 5. Artefactos esperados

```text
lab/parity/matrix/schema.json
lab/parity/matrix/manifest.yaml
lab/parity/matrix/runner.py
lab/parity/matrix/test_runner.py
lab/parity/matrix/fixtures/
00_nucleo/diagnosticos/paridade-matriz-p1138-baseline.md
00_nucleo/diagnosticos/paridade-p1138-backlog.md
```

Novos módulos auxiliares permanecem dentro de `lab/parity/matrix/` ou
`lab/parity/`; produção não importa `lab/`.

## 6. Critérios de aceitação

- [x] Há teste RED→GREEN para cada família de comparador nova.
- [x] Diagnósticos comparam mensagem e span, não somente exit code.
- [x] Valores eval são comparados por DTO tipado.
- [x] SVG e HTML usam árvore semântica documentada.
- [x] Geometria possui tolerâncias por campo, com unidade e fonte.
- [x] Raster produz métrica e artefacto de diferença reproduzível.
- [x] PDF compara observáveis extraídos, não bytes brutos.
- [x] Existe ao menos um caso executado em cada família A–D.
- [x] Todo resultado não-MATCH está listado integralmente no relatório.
- [x] Todo número decisório possui proveniência reproduzível.
- [x] O backlog P1139+ deriva das medições e separa língua de mecânica.
- [x] `python3 -m unittest lab/parity/matrix/test_runner.py` passa.
- [x] `cargo test --manifest-path lab/parity/Cargo.toml` passa.
- [x] `crystalline-lint .` não possui violações bloqueantes.
- [x] `git diff --check` passa.

## 7. Limite do passo

P1138 mede e prioriza; não promete paridade integral. A primeira correção de
produção decorrente desta baseline será um novo passo numerado, com ID estável
da matriz, medição anterior à decisão e L0 correspondente.
