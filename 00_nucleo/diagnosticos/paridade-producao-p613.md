# Paridade de produção — P613: Estabilidade do DocumentID do XMP

**Data:** 2026-07-08  
**Hash do commit de fecho:** `e59fd0c0f`  
**Materialization:** `00_nucleo/materialization/typst-passo-613.md`  

---

## 1. Verificação

### 1.1 Documento de teste

```typst
// /tmp/p613-mesmo.typ
Documento igual, compilado duas vezes.
```

### 1.2 Vanilla 0.15.0

```bash
lab/typst-original/target/release/typst compile /tmp/p613-mesmo.typ /tmp/p613-vanilla-1.pdf
sleep 2
lab/typst-original/target/release/typst compile /tmp/p613-mesmo.typ /tmp/p613-vanilla-2.pdf
```

```text
/tmp/p613-vanilla-1.pdf DocumentID: aJWAtX89vl1+HrOm5/XguQ==
/tmp/p613-vanilla-2.pdf DocumentID: mKKCKbNdebu0DxOLueG5aQ==
```

**Resultado:** o vanilla gera um `DocumentID` diferente a cada compilação do mesmo documento.

### 1.3 Cristalino

```bash
./target/release/typst /tmp/p613-mesmo.typ /tmp/p613-cristalino-1.pdf
sleep 2
./target/release/typst /tmp/p613-mesmo.typ /tmp/p613-cristalino-2.pdf
```

```text
/tmp/p613-cristalino-1.pdf DocumentID: EAX+nwcpSKtP07ZrYhbVZg==
/tmp/p613-cristalino-2.pdf DocumentID: EAX+nwcpSKtP07ZrYhbVZg==
```

**Resultado:** o cristalino mantém o mesmo `DocumentID` entre compilações do mesmo conteúdo.

---

## 2. Análise

A semântica do `DocumentID` diverge entre vanilla e cristalino:

| Ferramenta | Semântica do `DocumentID` |
|------------|---------------------------|
| Vanilla 0.15.0 | Aleatório / instável por compilação. Cada PDF do mesmo documento tem um ID diferente. |
| Cristalino (P612/P613) | Estável por conteúdo. Dois PDFs do mesmo documento partilham o mesmo `DocumentID`. |

Esta diferença é **semântica**, não morfológica. A forma do pacote XMP (namespaces, elementos, ordem, base64) permanece idêntica à do vanilla.

### 2.1 Por que manter a divergência

Um `DocumentID` estável por conteúdo tem mérito próprio: permite que ferramentas e utilizadores reconheçam duas cópias do mesmo documento como "o mesmo documento", algo que a semântica aleatória do vanilla não permite. O `InstanceID` do cristalino continua único por compilação (inclui timestamp), preservando a capacidade de distinguir versões.

---

## 3. Decisão

O cristalino **mantém** o `DocumentID` estável por conteúdo. A divergência face ao vanilla 0.15.0 é registada como **decisão intencional**, com razão escrita, no Prompt L0 `00_nucleo/prompts/infra/export/builder.md` (secção §P611, regra 8):

> Divergência intencional (P613): o vanilla 0.15.0 gera um `DocumentID` diferente a cada compilação do mesmo documento. O cristalino mantém `DocumentID` estável por conteúdo, por opção consciente: permite reconhecer duas cópias do mesmo documento como "o mesmo documento", o que o vanilla não permite. Esta diferença é semântica, não morfológica — a forma do pacote XMP permanece idêntica.

---

## 4. Validação

- Sonda directa confirmada: vanilla instável, cristalino estável.
- `crystalline-lint .` — `✓ No violations found`.

---

## 5. Conclusão

P613 está concluído. Confirmou-se que o vanilla 0.15.0 não mantém `DocumentID` estável, enquanto o cristalino o faz por design. A diferença foi documentada como divergência intencional no Prompt L0, em vez de ser deixada como paridade implícita.

---

## 6. Ligações

- Commit de fecho: `e59fd0c0f`
- Prompt L0: `00_nucleo/prompts/infra/export/builder.md`
- Materialization: `00_nucleo/materialization/typst-passo-613.md`
