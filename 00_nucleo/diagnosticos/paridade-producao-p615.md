# Paridade de produção — P615: Reverter DocumentID/InstanceID para aleatório

**Data:** 2026-07-08  
**Hash do commit de fecho:** `a9867122a`  
**Materialization:** `00_nucleo/materialization/typst-passo-615.md`  

---

## 1. Contexto

Em P612/P613, o cristalino passou a gerar `xmpMM:DocumentID` como um hash determinístico do conteúdo do documento, com a razão de "permitir reconhecer duas cópias do mesmo documento como o mesmo documento". P615 revisita essa decisão.

## 2. Problema na decisão anterior

A análise de P612/P613 estava incompleta. Um `DocumentID` baseado em hash de conteúdo falha em ambos os sentidos:

1. **Uma revisão real muda o conteúdo**, logo mudaria o hash — quebrando a continuidade que se pretendia manter.
2. **Documentos distintos com texto igual por coincidência** partilhariam o mesmo ID, misturando identidades que deviam ser distintas.

Sem estado entre compilações, não há forma correcta de detectar "o mesmo documento fonte recompilado" apenas a partir do conteúdo.

## 3. Decisão

Reverter para a abordagem do vanilla 0.15.0: gerar 16 bytes aleatórios em cada compilação, tanto para `DocumentID` como para `InstanceID`.

## 4. Implementação

### 4.1 Ficheiros alterados

- `03_infra/src/export/builder.rs`:
  - Remove `document_fingerprint` e `collect_xmp_fingerprint_text` (P612).
  - Adiciona `random_xmp_id_bytes` — gera 16 bytes pseudoaleatórios usando `getrandom` para o seed por processo e um PRNG determinístico para os valores subsequentes.
  - `xmp_instance_and_document_id` passa a devolver dois IDs aleatórios independentes em produção.
  - Em testes (`cfg!(test)` ou `CRYSTALLINE_PDF_FIXED_EPOCH`), mantém valores fixos.
- `03_infra/Cargo.toml` — adiciona `getrandom = "0.2"`.
- `Cargo.lock` — actualizado com a nova dependência.
- `00_nucleo/prompts/infra/export/builder.md` — secção §P611/P615 actualizada: explica porque o hash de conteúdo estava errado e confirma a aleatoriedade.

### 4.2 Estratégia de aleatoriedade

Para reconciliar paridade com o vanilla e determinismo dos testes:

- O seed é obtido via `getrandom` uma vez por processo.
- Dentro do mesmo processo, os IDs são determinísticos a partir desse seed (usando um contador atómico).
- Execuções separadas do compilador produzem seeds diferentes, logo IDs diferentes.
- Em testes, `cfg!(test)` força valores fixos.

Isto equivale a: "aleatório entre compilações, determinístico dentro da mesma execução do processo".

## 5. Validação

### 5.1 Mesmo documento, duas compilações

```bash
cat > /tmp/p615-mesmo.typ <<'EOF'
Documento igual, compilado duas vezes.
EOF
./target/release/typst /tmp/p615-mesmo.typ /tmp/p615-1.pdf
sleep 1
./target/release/typst /tmp/p615-mesmo.typ /tmp/p615-2.pdf
```

```text
=== /tmp/p615-1.pdf ===
DocumentID: 1f6iZUKREbulGkF3C43TPA==
=== /tmp/p615-2.pdf ===
DocumentID: 0/456iw+qsTgCSzIl1jaZw==
```

**Resultado:** `DocumentID` diferente em cada compilação, paridade real com o vanilla 0.15.0.

### 5.2 Testes automáticos

```bash
cargo test --workspace
```

Resultado: todos os testes passam.

```bash
crystalline-lint .
```

Resultado: `✓ No violations found`.

## 6. Conclusão

P615 está concluído. A decisão de P612/P613 foi revertida: `DocumentID` e `InstanceID` são agora aleatórios por compilação, alinhados com o vanilla 0.15.0 e com a prática ISO/Adobe para XMP. A nota de "divergência intencional" de P613 foi removida do Prompt L0.

---

## 7. Ligações

- Commit de fecho: `a9867122a`
- Prompt L0: `00_nucleo/prompts/infra/export/builder.md`
- Materialization: `00_nucleo/materialization/typst-passo-615.md`
