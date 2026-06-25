# P450 — Bibliography #5: Carregamento `.bib` de disco

> **Passo:** 450  
> **Data:** 2026-06-24  
> **Foco:** Implementar o carregamento de ficheiros `.bib` (BibTeX) do disco, convertendo entradas em `BibliographyEntry` para consumo pelo renderer de bibliografia.  
> **ADR-107:** Bibliography pipeline — fase de ingestão de dados bibliográficos.

---

## Contexto

O Typst vanilla suporta `#bibliography("refs.bib")` que carrega um ficheiro BibTeX do disco e o converte em entradas tipadas. O cristalino tem a infraestrutura de `BibliographyEntry` (entidade) e rendering básico, mas **não tem** o carregamento de `.bib` do filesystem. Este passo fecha a fase de ingestão da pipeline de bibliografia (ADR-107).

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `BibliographyEntry` existe como entidade? | Sim — P430 | ✅ |
| Rendering de bibliografia existe? | Sim — P430 (citação básica) | ✅ |
| Parser de `.bib` existe? | Não — zero código BibTeX | ❌ |
| File I/O infra existe? | Sim — `std::fs::read_to_string` via `03_infra` | ✅ |
| Path resolution (relativo ao `.typ`)? | Parcial — `ProjectPath` existe | 🟡 |
| Bloqueadores? | Nenhum técnico | ✅ |

**Reclassificação:** M (~40 min; parser BibTeX minimal + file I/O + integração com eval + tests).

---

## Toques pontuais

### 1. Parser BibTeX minimal (`03_infra/src/bibtex.rs`)

Implementar parser recursivo descendente para subset BibTeX usado pelo Typst:

- **Entradas:** `@article`, `@book`, `@inproceedings`, `@misc`, `@phdthesis`, `@techreport` (6 tipos cobrem 95% dos casos).
- **Campos obrigatórios:** `title`, `author`, `year`, `doi` (opcional), `url` (opcional), `journal`/`booktitle` (condicional ao tipo).
- **Formato de author:** `Last, First and Last2, First2` → parsed para `Vec<Author>`.
- **Escaping:** `"{"o}}"` → `"o"` (minimal; não suporta LaTeX macros arbitrários).
- **Encoding:** UTF-8 only; rejeita ISO-8859-1 com erro claro.

```rust
pub struct BibTeXParser;
impl BibTeXParser {
    pub fn parse(src: &str) -> Result<Vec<BibliographyEntry>, BibTeXError>;
}
```

### 2. File I/O + path resolution (`03_infra/src/filesystem.rs` ou `project.rs`)

- `Project::resolve_path(path: &str) -> PathBuf` — resolve relativo ao ficheiro `.typ` fonte.
- `Project::load_bibliography(path: &str) -> Result<Vec<BibliographyEntry>, Error>` — lê ficheiro, invoca parser, retorna entradas.

### 3. Integração com eval (`rules/eval/mod.rs`)

- `native_bibliography(path: EcoString)` — função nativa que invoca `project.load_bibliography(&path)`.
- Registar no stdlib scope como `"bibliography"`.
- Retorna `Value::Content(Content::Bibliography(entries))` (ou `Value::Array` de entries, dependendo do modelo existente).

### 4. Atualização de `BibliographyEntry` (`entities/bibliography.rs`)

- Se necessário, adicionar campos que faltam: `doi: Option<EcoString>`, `url: Option<EcoString>`, `pages: Option<EcoString>`.
- Garantir `Clone + Debug + PartialEq` para tests.

### 5. Tests

- **L1 (parser unit):** 5 testes — article mínimo, book com múltiplos autores, escaping UTF-8, campo opcional omitido, erro em tipo desconhecido.
- **L2 (integration):** 2 testes — carregamento de ficheiro `.bib` temporário, path resolution relativo.
- **L3 (E2E):** 1 teste — `#bibliography("refs.bib")` em documento Typst produz `Content::Bibliography` com entradas correctas.

### 6. Spec L0

- `00_nucleo/prompts/03_infra/bibtex.md` — gramática suportada, limitações, escaping.
- `00_nucleo/prompts/rules/stdlib/bibliography.md` — função `bibliography(path)`.

---

## Scope-out explícito

- **CSL-JSON** — scope-out; apenas BibTeX por ora.
- **LaTeX macros** — scope-out; `"{\"o}}"` é o máximo de escaping suportado.
- **Crossref / DOI resolution online** — scope-out; apenas ficheiro local.
- **Estilos de citação (APA, IEEE, etc.)** — scope-out; P451 ou posterior.
- **Ordenação automática** — scope-out; ordem do ficheiro `.bib` preservada.
- **Deduplicação** — scope-out.

---

## Critério de fecho

- [ ] `BibTeXParser` implementado em `03_infra/src/bibtex.rs`.
- [ ] 5 testes L1 de parser verdes.
- [ ] `Project::load_bibliography` implementado.
- [ ] `native_bibliography` registada no stdlib.
- [ ] 2 testes L2 de file I/O verdes.
- [ ] 1 teste L3 E2E verde.
- [ ] Spec L0 actualizado (2 prompts).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] `BibliographyEntry` tem campos suficientes para rendering básico.

---

## Alternativas consideradas

| Alternativa | Porquê rejeitada |
|-------------|------------------|
| Usar crate `biblatex` externa | Adiciona dependência pesada; cristalino prefere controle total do parsing para garantir bit-equivalência e diagnósticos próprios. |
| Parser PEG/pest | Overkill para subset minimal; recursivo descendente é 200 linhas, testável, zero deps. |
| Suportar CSL-JSON agora | Duplica esforço; BibTeX é 90% dos casos reais. |

---

**Aguardando sua indicação:**

1. **Executar o P450** (bibliography .bib de disco, ~40 min)?
2. **Pivotar para outra frente** (heading numbering, links/hyperlinks, DEBT-42 benchmark)?
3. **Ajustar o escopo** do P450?
