# Relatório de Paridade — P779

**Data:** 2026-07-16
**Passo:** P779
**Objetivo:** Auditar e corrigir `validate.py` para usar a metodologia de rasterização do projecto; registar a lição processual.

---

## Estado base da medição

- **Commit base:** `18d2f9d780dcad74caa6d4473dc64b81d9ee8f5d` (P778)
- **Ficheiros auditados/corrigidos:**
  - `temp_p776/validate.py` (cópia de trabalho)
  - `lab/parity/tools/validate_exif_orientation.py` (versão canónica promovida)
  - `00_nucleo/handoff-novo-chat-p762.md` (lição registada)
- **Ferramenta de comparação fixada:** `mutool draw -r 300` + `compare -metric AE`.

---

## Sonda — o que `validate.py` fazia

Ficheiro: `temp_p776/validate.py`, linhas 72–80.

```python
def rasterize(pdf, png):
    r = run(["pdftoppm", "-png", "-r", "150", str(pdf), str(png.with_suffix(''))])
```

**Problemas encontrados:**
1. Usava `pdftoppm` em vez de `mutool draw`.
2. Usava resolução `150 dpi` em vez de `300 dpi`.
3. Não documentava a escolha nem fazia referência à convenção do projecto.

Esta combinação foi a fonte dos resíduos `AE=195/138/0` reportados em P776/P777, que o P778 provou serem artefactos de medição (com `mutool draw -r 300`, `pdftoppm -r 300` ou `gs -r 300` o AE é 0).

---

## Correcção

### Versão canónica promovida

Criado `lab/parity/tools/validate_exif_orientation.py` com:
- Constante `RASTERIZER_CMD = ["mutool", "draw", "-r", "300"]`.
- Comentário de convenção no docstring, referenciando o relatório P778.
- Estrutura reutilizável e auditável.

### Cópia de trabalho atualizada

`temp_p776/validate.py` foi corrigido para usar a mesma metodologia e aponta para a versão canónica. A função `rasterize` passou a ser:

```python
def rasterize(pdf, png):
    cmd = list(RASTERIZER_CMD) + ["-o", str(png), str(pdf), "1"]
    r = run(cmd)
```

---

## Validação

Execução de `temp_p776/validate.py` após a correcção:

```bash
cd temp_p776 && ../lab/.venv/bin/python validate.py
```

| Orientação | AE cristalino vs vanilla |
|-----------|--------------------------|
| 1         | 0                        |
| 2         | 0                        |
| 3         | 0                        |
| 4         | 0                        |
| 5         | 0                        |
| 6         | 0                        |
| 7         | 0                        |
| 8         | 0                        |

**Resultado:** todas as orientações passam com AE=0, consistente com as medições manuais do P778.

---

## Registo da lição

Adicionada regra 10 ao `00_nucleo/handoff-novo-chat-p762.md`, secção "Regras/ADRs em vigor":

> **Auditar scripts de validação antes de confiar nos números** — qualquer `validate.py` ou script equivalente usado para gerar números de um relatório de paridade tem de ser lido e confirmado, não aceite pelo nome. A ferramenta e resolução de rasterização têm de bater com a convenção do projecto (`mutool draw -r 300` para comparação de imagens) e devem estar fixadas no próprio script (constante/comentário), não deixadas à escolha de quem o escreveu num passo específico. (Lição de P778/P779: `pdftoppm -r 150` num script não auditado gerou resíduos de AE que não existiam com a metodologia padrão.)

---

## Decisões

1. **`validate.py` é promovido a ferramenta permanente** do projecto, com a versão canónica em `lab/parity/tools/validate_exif_orientation.py`.
2. **A cópia em `temp_p776/validate.py` mantém-se** como ponto de entrada imediato para reproduzir a validação do contexto P776/P777/P778/P779, mas deve replicar a convenção do script canónico.
3. **A linha de trabalho P769–P779 fecha aqui.** O próximo passo sugerido no handoff é retomar a varredura da stdlib em P772a (lote 3).
