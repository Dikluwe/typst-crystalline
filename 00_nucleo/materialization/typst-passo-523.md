---
# P523 — Trilha 6: Polimento de CFF subsetting (documentação, teste, descritor PDF)

> **Passo:** 523
> **Data:** 2026-07-01
> **Foco:** Fechar a Trilha 6 (CFF subsetting), confirmada funcional pela sonda P522. (1) Corrigir a narrativa "CFF scope-out XL" em todos os locais onde existe, não só um comentário; (2) adicionar teste de export CFF ao corpus, com fonte em fixture (não dependente do sistema); (3) investigar descritor PDF `CID TrueType` vs `CID Type 0C` sem assumir sinal ou complexidade de fix a priori.
> **Tipo:** Implementação + Documentação + Sonda condicional.
> **Tamanho:** M (~50 min).
> **ADR-0108 EM VIGOR** — medir antes de decidir. **O critério de kerning da Sub-tarefa 2 é derivado empiricamente de um caso já validado, não assumido por raciocínio a priori** — a primeira versão de P520 já cometeu esse erro uma vez com o sinal do `TJ`.
> **ADR-0109 EM VIGOR** — atomização; cada sub-tarefa independente.
> **ADR-0107 EM VIGOR** — descritor PDF é mecânica, não linguagem.
> **Dependências:** P522 (sonda CFF), P516, P520, P521.
> **Trilha:** 6 — CFF subsetting (fechamento).

---

## Contexto

P522 confirmou: CFF subsetting já funciona via `oxifont-subset` 0.2.0, sem corrupção, sem panic, sem `.notdef`. A alegação do handoff ("fallback ativo, XL-size") estava errada. Três itens de polimento restam — mas dois deles têm armadilhas concretas que a versão anterior deste passo não evitava.

---

## Sub-tarefa 1 — Corrigir a narrativa "CFF é scope-out XL" em todos os locais, não só um comentário

### 1.1 — Localizar todas as ocorrências

```bash
grep -rln "CFF.*[Ss]cope-out\|CFF.*fallback\|CFF.*XL" \
  00_nucleo/ 03_infra/src/ 01_core/src/ --include="*.md" --include="*.rs"
```

Locais esperados a verificar, além do comentário em `subset.rs:31`:
- Documento de handoff (secção 5.1 — "CFF subsetting ⏸️ Scope-out. Fallback ativo, XL-size").
- `00_nucleo/diagnosticos/debt/DEBT.md`, se houver entrada relacionada.
- Qualquer L0 prompt em `00_nucleo/prompts/infra/export/` que mencione CFF como não suportado.
- README ou documento de estado geral do projecto, se existir.

### 1.2 — Fix do comentário em `subset.rs`

```rust
/// Subset uma fonte OpenType (TrueType `glyf` ou CFF/CFF2) para um
/// conjunto de glyph IDs. O `oxifont-subset` detecta o formato internamente
/// (ver src/cff.rs::rewrite_cff) e aplica o caminho apropriado.
/// Retorna `None` apenas se o `font_data` for inválido ou o subsetter falhar.
pub fn subset_font_with_mapping(...) -> Option<SubsetResult> {
```

### 1.3 — Fix de cada ocorrência encontrada em 1.1

Para cada ficheiro `.md` identificado, actualizar a afirmação para reflectir o resultado de P522: CFF é suportado; o trabalho restante (se houver, pós Sub-tarefa 3) é polimento de descritor PDF, não implementação de subsetting.

### Critério de fecho

- [ ] Todas as ocorrências de "CFF scope-out/fallback/XL" localizadas com `grep`, não só a de `subset.rs`.
- [ ] Cada uma corrigida para reflectir P522.
- [ ] `cargo test --workspace` passa (sem regressão — mudança é só documentação).

---

## Sub-tarefa 2 — Teste de CFF no corpus, com fonte em fixture

### 2.1 — Fonte de teste como fixture do repositório, não caminho de sistema

**Não usar `/usr/share/fonts/...` em nenhum teste permanente.** Copiar a fonte de teste (confirmada CFF por P522 — Nimbus Sans, C059, ou FreeSerif, licenças URW/GNU FreeFont compatíveis com redistribuição) para dentro do repositório:

```bash
mkdir -p 03_infra/fixtures/fonts/
cp /usr/share/fonts/opentype/urw-base35/NimbusSans-Regular.otf \
   03_infra/fixtures/fonts/NimbusSans-Regular.otf
```

Confirmar a licença da fonte antes de commitar (URW Base 35 é AGPL/GPL com excepção de fontes — verificar o ficheiro de licença junto à fonte no sistema, `grep -l "license\|LICENSE" /usr/share/fonts/opentype/urw-base35/` ou equivalente, e incluir nota de licença junto à fixture).

### 2.2 — Documento de teste no corpus

`lab/parity/corpus/p523/test-cff-nimbus.typ`:

```typst
#set text(font: "Nimbus Sans", size: 12pt)
Hello world. The five boxing wizards jump quickly.

#set text(font: "Nimbus Sans", size: 24pt)
ffi fl fi AV
```

### 2.3 — Critérios de validação (sinal de kerning derivado, não assumido)

1. **Compilação:** exit code 0.
2. **Fonte embebida:** `pdffonts` confirma Nimbus Sans presente.
3. **Tamanho:** PDF sensivelmente menor que a fonte completa (82 KB) — subsetada, não embebida inteira.
4. **Texto extraível:** `pdftotext` devolve o texto completo, incluindo `ffi`, `fl`, `fi`.
5. **Ligatures:** glifos de `ffi`/`fl`/`fi` são distintos de `.notdef` (mesma verificação estrutural do P522 — extrair a fonte do PDF e confirmar via `fontTools` que os glyph IDs usados existem e não caem em `.notdef`).
6. **Kerning — critério derivado, não assumido:**

   Antes de decidir que sinal esperar para `AV` com Nimbus Sans (CFF), gerar o mesmo par `AV` com uma fonte **TrueType já validada** em P520/P521 (ex.: Noto Sans, cujo `TJ` para `AV` já foi confirmado correcto visualmente):

   ```bash
   ./target/release/typst lab/parity/corpus/p520/test-kerning.typ /tmp/av-truetype-baseline.pdf
   mutool show /tmp/av-truetype-baseline.pdf 4 | grep TJ
   ```

   Registar o sinal e a magnitude aproximada do delta observado para `AV` nesse caso **já confirmado correcto**. Usar esse valor como referência — não a convenção descrita em prosa no relatório de P520, que pode não corresponder exactamente ao código real (foi exactamente esse tipo de assunção por raciocínio que gerou o bug corrigido na primeira versão de P520). Depois, comparar o `TJ` do `AV` em Nimbus Sans (CFF) contra essa referência: mesmo sinal e ordem de grandeza compatível com o kerning declarado na fonte, é suficiente — não é preciso o valor exacto, porque fontes diferentes têm tabelas de kerning diferentes.

### 2.4 — Teste unitário, com skip gracioso se a fixture não existir

```rust
#[test]
fn subset_cff_nimbus_sans() {
    let fixture_path = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts/NimbusSans-Regular.otf");
    let font_data = match std::fs::read(fixture_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("SKIP subset_cff_nimbus_sans: fixture não encontrada em {fixture_path}: {e}");
            return; // skip gracioso, não panic — a fixture devia existir após 2.1,
                     // mas o teste não deve derrubar o CI por um problema de checkout/LFS.
        }
    };
    let result = subset_font_with_mapping(&font_data, &[0, 1, 2, 3]).unwrap();
    let font = font::parse(&result.font_data).unwrap();
    assert!(font.has_table(b"CFF "), "fonte subsetada deve preservar a tabela CFF");
}
```

**Nota:** usar `env!("CARGO_MANIFEST_DIR")` em vez de caminho relativo cru, para o teste funcionar independentemente do directório de onde `cargo test` é invocado.

### Critério de fecho

- [ ] Fonte CFF copiada para `03_infra/fixtures/fonts/`, com licença verificada e anotada.
- [ ] Documento de corpus criado em `lab/parity/corpus/p523/`.
- [ ] Sinal de kerning esperado derivado de um caso TrueType já validado, não assumido do texto do relatório P520.
- [ ] Teste unitário usa a fixture do repositório, com skip gracioso (não `.unwrap()` num caminho de sistema).
- [ ] Todos os 6 critérios de 2.3 confirmados.

---

## Sub-tarefa 3 — Descritor PDF `CID TrueType` vs `CID Type 0C` — sonda, sem assumir magnitude do fix

### 3.1 — Sonda de compatibilidade

```bash
verapdf /tmp/cff-cristalino.pdf 2>/dev/null | grep -i "CIDFontType\|CFF\|font"
```

**Se `verapdf` não estiver disponível**, não pular a verificação silenciosamente — registar essa limitação explicitamente e usar inspecção manual do `FontDescriptor`/`Subtype` como alternativa qualitativa, deixando claro no relatório que a validação formal contra a spec PDF não foi possível neste ambiente.

### 3.2 — Decisão, com estimativa de magnitude corrigida

| Cenário | Decisão |
|---------|---------|
| Nenhum leitor testado rejeita o PDF; `verapdf` (se disponível) não reporta erro crítico | Documentar como scope-out mecânico (ADR-0107). Fechar Trilha 6 aqui. |
| `verapdf` reporta erro crítico de conformidade | **Não implementar o fix neste passo.** `CIDFontType0C` requer embeber o programa CFF via `FontFile3` em vez de `FontFile2` — é uma mudança de estrutura de embedding, não uma troca de string em `/Subtype`. Isso tem potencial de interagir com o modelo de delta de kerning, o ToUnicode CMap, e o mapeamento de glyph_id para ligatures, todos validados em P520/P521 assumindo o caminho `CIDFontType2`/`FontFile2`. Abrir como passo dedicado (P524) com sonda própria antes de tocar código. |

### Critério de fecho

- [ ] `Subtype` confirmado em ambos os PDFs (cristalino e vanilla).
- [ ] Verificação de conformidade tentada; se `verapdf` indisponível, isso é registado como limitação, não omitido.
- [ ] Decisão tomada com a magnitude de fix corrigida (S só se for troca cosmética confirmada; caso contrário, passo próprio).

---

## Critério de fecho do passo

- [ ] Sub-tarefa 1: narrativa "CFF scope-out" corrigida em todos os locais encontrados por `grep`, não só um comentário.
- [ ] Sub-tarefa 2: teste CFF no corpus, fonte em fixture do repositório, sinal de kerning derivado empiricamente de caso já validado.
- [ ] Sub-tarefa 3: descritor investigado; se fix necessário, não implementado aqui — abre P524 dedicado.
- [ ] `cargo test --workspace` passa.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p523.md`.
- [ ] Trilha 6 marcada como fechada (subsetting funcional) ou "fechada com P524 pendente" (se descritor exigir fix estrutural).

---

## Próximo passo

- **Se Sub-tarefa 3 não exigir fix:** Trilha 6 fechada. Retomar opções do handoff (Lookahead, publicação, optimização).
- **Se Sub-tarefa 3 exigir fix estrutural do descritor:** P524 dedicado, com sonda própria sobre o impacto em kerning/ToUnicode/ligatures antes de qualquer edição — não assumir que é troca simples de string.
