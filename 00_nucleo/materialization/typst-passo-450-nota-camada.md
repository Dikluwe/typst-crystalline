# Nota de reconciliação de camada — P450 Bibliography/Cite

**Data:** 2026-06-24  
**Passo:** 450  
**ADR-0114:** Aplica-se — reconciliação de camada não declarada na spec original.

---

## O que a spec P450 propunha

A spec P450 previa o parser BibTeX na camada **L3** (`03_infra`), com um wrapper de I/O em L3 e a lógica de parsing potencialmente delegada a L1.

## O que foi implementado

A implementação real distribuiu-se assim:

```text
L3 (03_infra)
  SystemWorld::load_bibliography(path) → lê ficheiro → bytes → Str

L1 (01_core)
  parse_bibtex(&str) → Vec<BibliographyEntry>
  eval de #bibliography("refs.bib") → chama world.load_bibliography
```

Ou seja, **o parser puro vive em L1** (`01_core/src/engine/eval/bibliography.rs` e helpers de parsing BibTeX), enquanto **o I/O de ficheiro vive em L3** (`SystemWorld`).

---

## Porque a mudança de camada é coerente

1. **Parser puro em L1 é consistente com P388** (`bibliography`/`cite` foi desenhado como feature de L1 desde o início; a dependência `hayagriva` é externa, mas a lógica de adaptação é domínio de L1).
2. **I/O em L3 é correcto** — `World` é a fronteira de sistema; carregar bytes de ficheiro é responsabilidade da infraestrutura, não do core semântico.
3. **Zero violação de fronteiras** — L1 não faz syscall directa; L3 não faz parsing semântico. A fronteira L1/L3 é mantida.

---

## Arquitectura final formalizada

```text
L3 (03_infra)  SystemWorld::load_bibliography(path) → bytes → L1
L1 (01_core)   parse_bibtex(&str) → Vec<BibliographyEntry>
```

Esta nota reconcilia a spec original com a implementação efectiva e encerra a pendência documental.
