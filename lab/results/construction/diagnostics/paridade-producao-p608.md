# Paridade de Produção — P608

**Data do relatório:** 2026-07-08
**Passo:** 608
**Foco:** Verificar se a fusão de blocos `BT...ET` consecutivos ainda é uma disparidade com o vanilla 0.15.0.

---

## Resumo executivo

P534 registou como scope-out a fusão de blocos `BT...ET`: quando há fallback de fonte a meio de uma linha, o cristalino fecha o bloco actual e abre um novo para cada troca de fonte, enquanto o vanilla funde runs adjacentes num único bloco `BT...ET`. Este passo reconfirmou o sintoma num documento de teste com texto latim, CJK e árabe.

**Decisão:** mantém-se como **scope-out deliberado**. O texto renderiza correctamente; trata-se de uma optimização de export que reduziria o número de blocos PDF e o tamanho do ficheiro. A correcção exigiria alterar o emissor de texto para fundir runs adjacentes com fontes diferentes dentro do mesmo bloco `BT...ET`, o que está fora do scope actual.

---

## Proveniência

- **Hash base:** `49f702f886d7f31bcd257871fdc182fca5d59212`
- **Data/hora:** 2026-07-08T01:05-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst` (P606)
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `mutool show`, `pdftotext`

---

## Documento de teste

`/tmp/p608-fallback.typ`:

```typst
Hello 你好 مرحبا world, more latin text after the fallback scripts.
```

Força o fallback de fonte: latim, CJK (你好) e árabe (مرحبا) na mesma linha.

---

## Resultados

### Contagem de blocos `BT...ET`

Contagem feita nos content streams das páginas (`mutool show <pdf> pages` → objecto da página → `/Contents`) com regex `\bBT\b` / `\bET\b`.

| Compilador | Blocos `BT` | Blocos `ET` |
|------------|-------------|-------------|
| Cristalino | 11 | 11 |
| Vanilla 0.15.0 | 5 | 5 |

O cristalino gera mais do dobro dos blocos de texto do vanilla para este documento.

### Exemplo de estrutura

**Cristalino** (objecto 4, excerpto):

```text
BT /F1 11.0 Tf 70.867 759.990 Td [ <0003> 0 <0008> ... ] TJ ET
BT /F2 11.0 Tf 98.055 759.990 Td [ <2703> 0 <3790> 0 ] TJ ET
BT /F3 11.0 Tf 122.805 759.990 Td [ <001B> 0 ... ] TJ ET
BT /F1 11.0 Tf 158.555 759.990 Td [ ... ] TJ ET
...
```

Cada segmento de texto com uma fonte diferente (ou mesmo com a mesma fonte mas em runs separados) é envolvido no seu próprio par `BT...ET`.

**Vanilla** (objecto 27, excerpto):

```text
/Span<</MCID 0>>BDC q 1 0 0 -1 70.86614 762.96063 cm ...
BT 0 Tr /f0 11 Tf 1 0 0 -1 0 0 Tm [(...)] TJ ET
Q EMC
/Span<</MCID 1>>BDC ... BT ... ET Q EMC
...
```

O vanilla agrupa mais texto por bloco `BT...ET`, mudando a fonte dentro do mesmo bloco quando necessário.

### Tamanho de ficheiro

```text
-rw-rw-r-- 1 dikluwe dikluwe 15653692 jul  8 08:01 /tmp/p608.pdf
-rw-rw-r-- 1 dikluwe dikluwe    12015 jul  8 08:01 /tmp/p608-vanilla.pdf
```

**Nota:** a diferença massiva de tamanho (15,7 MB vs 12 KB) não é causada pelos blocos `BT...ET` em si, mas pelo facto de o cristalino embutir fontes completas em vez de as sub-settingar (scope-out separado, ver P515/P519). O impacto directo da fusão de blocos é da ordem de centenas de bytes neste documento.

### Correcção visual

`pdftotext` produz o mesmo texto nos dois PDFs:

```text
Hello 你好 ‫ مرحبا‬world, more latin text after the fallback scripts.
```

A disparidade é puramente mecânica / de optimização.

---

## Análise

1. **O sintoma ainda existe.** Apesar das consolidações de P543, P548, P558, P591 e P593 na área de export de texto, nenhuma alterou a granularidade de emissão dos blocos `BT...ET`.

2. **A diferença é do mesmo tipo que P534.** O cristalino ainda emite um bloco por run de texto (frequentemente por troca de fonte), enquanto o vanilla funde runs adjacentes.

3. **Não é uma regressão funcional.** O texto é extraído correctamente e renderiza igual.

4. **O ganho potencial é pequeno em comparação com outras optimizações.** O verdadeiro problema de tamanho de ficheiro é a falta de font subsetting. A fusão de blocos `BT...ET` melhoraria apenas marginalmente o tamanho.

---

## Decisão

**Manter como scope-out deliberado.**

Razão actualizada (P608): a fusão de blocos `BT...ET` consecutivos é uma optimização de export. O texto renderiza correctamente; a correcção exigiria reestruturar o emissor de texto para manter um único bloco `BT...ET` activo ao longo de vários runs com fontes diferentes, gerenciando mudanças de fonte (`Tf`), cores (`scn`) e posições (`Tm`) dentro do mesmo bloco. Isso está fora do scope actual do projecto.

Se no futuro o subsetting de fontes for implementado, a fusão de blocos `BT...ET` torna-se uma optimização natural seguinte, porque aí o ganho de tamanho passaria a ser significativo.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - A entrada "Fusão de blocos `BT...ET`" foi movida da secção "Corrigido ao longo desta conversa" (onde estava incorrectamente listada com uma nota a pedir confirmação) para a secção "Scope-out deliberado", com a razão actualizada em P608.

- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - Secção 2.2 (Exportação / texto): entrada "Fusão de blocos `BT...ET` consecutivos" actualizada para **Scope-out confirmado** em P608.

---

## Critérios de fecho do passo

- [x] Sintoma confirmado directamente, com números (11 vs 5 blocos `BT...ET`).
- [x] Decisão registada: mantém-se scope-out, com razão actualizada.
- [x] Relatório escrito com proveniência.
- [x] Listas de disparidades actualizadas, removendo a nota de "confirmar se ainda se aplica".

---

## Ligações

- `00_nucleo/materialization/typst-passo-608.md` — passo que originou a verificação.
- `00_nucleo/diagnosticos/paridade-producao-p534.md` — passo original que registou o scope-out.
- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md` — inventário actualizado.
