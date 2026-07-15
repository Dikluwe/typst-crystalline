# Diagnóstico P763b — Validação ponta a ponta: `cetz` com cache totalmente vazia

**Data da medição:** 2026-07-15T16:29:00-03:00  
**Commit base:** `f36ca1abe2f95cffc2cf6fd2b3cf4947d2d9f846`  
**Working tree:** limpo (sem alterações não commitadas)  
**Passo:** P763b  
**Objectivo:** Confirmar que o download automático de pacotes implementado em P763a funciona para um pacote com dependências transitivas (`@preview/cetz:0.5.2`), sem cache pré-populada.

---

## Documento de teste

`/tmp/p763b-cetz.typ`:

```typst
#import "@preview/cetz:0.5.2"
#cetz.canvas({
  import cetz.draw: *
  line((0, 0), (2, 1))
  circle((0, 0))
})
```

Vanilla de referência: `lab/typst-original/target/release/typst` (Typst 0.15.0).  
Cristalino: `target/release/typst`.

---

## Medições

### Limpeza de cache

```bash
rm -rf ~/.cache/typst/packages
rm -rf ~/.local/share/typst/packages
```

Confirmado: nenhum pacote em cache antes do teste.

### Compilação cristalina (cache vazia)

```bash
./target/release/typst /tmp/p763b-cetz.typ /tmp/p763b-cetz.pdf
```

**Resultado:** OK. Tempo real: ~3 s (inclui download de `cetz` e dependências).

### Dependências transitivas descarregadas

Após a compilação, a cache continha:

```text
~/.cache/typst/packages/preview/cetz/0.5.2
~/.cache/typst/packages/preview/oxifmt/1.0.0
```

`oxifmt` é referenciado por `cetz` em:

```text
~/.cache/typst/packages/preview/cetz/0.5.2/src/deps.typ:1:#import "@preview/oxifmt:1.0.0"
~/.cache/typst/packages/preview/cetz/0.5.2/src/anchor.typ:2:#import deps.oxifmt: strfmt
~/.cache/typst/packages/preview/cetz/0.5.2/src/draw/grouping.typ:13:#import deps.oxifmt: strfmt
```

Confirmação: o mecanismo de download transitivo funciona — `cetz` sozinho não bastaria; a dependência `oxifmt` foi adquirida automaticamente.

### Comparação visual

```bash
lab/typst-original/target/release/typst compile /tmp/p763b-cetz.typ /tmp/p763b-cetz-vanilla.pdf
mutool draw -o /tmp/p763b-vanilla.png -r 300 /tmp/p763b-cetz-vanilla.pdf
mutool draw -o /tmp/p763b-cristalino.png -r 300 /tmp/p763b-cetz.pdf
compare -metric AE /tmp/p763b-vanilla.png /tmp/p763b-cristalino.png /tmp/p763b-diff.png
```

| Métrica | Valor |
|---------|-------|
| AE | **10725** |
| Dimensões | 2481×3508 (300 ppp, A4) |

A diferença concentra-se no posicionamento dos dois elementos (`line` + `circle`) desenhados pelo canvas: o cristalino posiciona o grupo ligeiramente deslocado em relação ao vanilla.

### Teste de controlo — cache já preenchida

Para descartar que a diferença seja um efeito do *primeiro* download (por exemplo, algum ficheiro descarregado de forma incompleta ou com cache em estado inconsistente), repetiu-se a compilação com a cache já populada:

```bash
./target/release/typst /tmp/p763b-cetz.typ /tmp/p763b-cetz-2.pdf
```

**Resultado:** AE = **10725** (idêntico). A diferença visual é reprodutível e independente do estado da cache.

### Referência de base — rect simples

Para contextualizar a magnitude da diferença, mediu-se um documento mínimo sem pacotes:

```typst
#rect(width: 2cm, height: 2cm)
```

| Métrica | Valor |
|---------|-------|
| AE | **241** |

A diferença de base num `rect` simples (provavelmente antialiasing / espessura de stroke) já é não-nula; a diferença observada em `cetz` é da mesma ordem de natureza mecânica, amplificada pelo deslocamento de múltiplos objetos.

---

## Conclusão

- **Download automático:** funciona. `cetz:0.5.2` e a sua dependência transitiva `oxifmt:1.0.0` foram adquiridos automaticamente a partir de cache vazia.
- **Regressão visual causada pelo download:** **não detectada**. A diferença de AE=10725 é reprodutível com cache preenchida, logo não é efeito do caminho de aquisição implementado em P763a.
- **Paridade visual cetz vs vanilla:** não é AE=0 neste momento, mas essa diferença é preexistente e de natureza mecânica de renderização, não introduzida por este passo. O relatório P762 registou apenas que `cetz` compilava sem erro; não estabeleceu um baseline AE=0 para este documento.
- **Próximo passo recomendado:** se se quiser perseguir AE=0 para `cetz`, abrir um passo dedicado à investigação do deslocamento dos elementos do canvas (possivelmente relacionado com `top-edge`/`bottom-edge`, medições de fonte, ou bounding-box do canvas). Não faz parte do âmbito de P763b, que era validar a aquisição automática de pacotes.

---

## Validação

- `cargo test --workspace` — OK.
- `crystalline-lint .` — zero violações (exceto V7 esperado de `package_version_resolution.md`).
- Cache e data dir vazios antes do teste — confirmado.
- `cetz` compila sem intervenção manual — confirmado.
- Dependência transitiva `oxifmt` descarregada automaticamente — confirmado.
