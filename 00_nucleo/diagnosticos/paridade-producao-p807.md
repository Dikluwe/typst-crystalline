# Relatório de Verificação — Passo 807: `pdf::attach` — decisão de escopo (achado P798 #11)

**Data:** 2026-07-21
**Status:** Concluído — **Opção B (decisão do dono): scope-out mantido, formalizado como DEBT-66**
**Proveniência da Medição:**
- **Commit Base:** `0661aef91` (HEAD) + working tree P799–P806
- **Hora da Medição:** 2026-07-21 ~17:50 (-0300)
- **Entregável (por decisão do dono):** entrada **DEBT-66** em `00_nucleo/diagnosticos/debt/DEBT.md` — sem código novo

---

## 1. O Problema Relatado

Achado #11 de P798: `#pdf.attach("dummy.txt")` — o vanilla embute o ficheiro no PDF; o cristalino rejeita com erro explícito de scope-out. O passo não era de correcção directa: pedia levantamento + **decisão do dono** antes de qualquer implementação.

## 2. Levantamento (Passo 1 do prompt)

- **Comportamento actual (medido)**: cristalino `error: pdf.attach: o exportador PDF cristalino não suporta ficheiros embutidos (scope-out)` (`01_core/src/engine/stdlib/pdf.rs::native_pdf_attach`). Vanilla gera `/EmbeddedFiles << /Names [ (dummy.txt) 6 0 R ] >>` no catálogo + objecto `/Filespec` (verificado com `mutool show`).
- **Mecanismo vanilla** (`crates/typst-library/src/pdf/attach.rs`): `AttachElem { path, data, relationship?, mime-type?, description? }` — lê os bytes (ou recebe `Bytes`) e o exportador escreve embedded file stream + `/Filespec` + name tree. **Sem dependência externa especial** (bytes crus).
- **Ponto de rejeição cristalino**: `native_pdf_attach` — erro deliberado, documentado no header de linhagem e no L0 `stdlib/pdf.md`.
- **Achado processual (Passo 1.4 do prompt)**: o scope-out **nunca foi formalizado** — não existe ADR nem entrada no inventário de dívidas (`00_nucleo/diagnosticos/debt/DEBT.md`; o ficheiro `00_nucleo/DEBT.md` que o prompt referia não existe — o inventário vive em `diagnosticos/debt/`). Esta foi em si uma descoberta registada.
- **Estimativa de peso**: sem dependência nova; exige novo canal L1→L3 (variant `Content` ou side-channel até ao exportador) + secção de export (embedded file stream, `/Filespec`, `/Names /EmbeddedFiles`) + testes — esforço estimado 1–2 passos. Infra reutilizável existe (`PdfBuilder` já escreve streams, dicionários e catálogo).

## 3. Decisão (Passo 2 — dono do projecto)

Apresentadas as duas opções com o levantamento, o dono escolheu **Opção B — manter o scope-out**, alinhado com os precedentes `image::svg` (P772k) e PDF-como-imagem (P781, peso de dependência).

**Formalização**: nova entrada **DEBT-66** em `00_nucleo/diagnosticos/debt/DEBT.md` (com o achado processual, a decisão, a estimativa de peso e o critério de reabertura — pedido explícito do dono ou caso de uso real em corpus, ex.: ZUGFeRD/Factur-X) + nota de saldo no cabeçalho (6 → 7 abertos).

## 4. Testes Automatizados Persistidos

Nenhum (decisão sem código — conforme o Passo 5 do prompt para a Opção B).

## 5. Verificação de Sucesso do Workspace

```
crystalline-lint . → exit 0 (após a entrada DEBT-66)
```

Verificação final do workspace completo (2026-07-21 ~18:00): typst-core 4336/1i, typst-infra 657/5i, typst-shell 33, CLI bin 2, cli.rs 29, crystalline_lint 2 — zero falhas.
